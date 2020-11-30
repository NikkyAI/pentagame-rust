import {
  /* webpackMode: "eager" */
  PentaMath,
} from "./pentagame.js";
import {
  /* webpackMode: "eager" */
  SVG,
} from "@svgdotjs/svg.js";
import {
  /* webpackMode: "eager" */
  getJSONP,
  download,
  create_alert,
} from "./utils.js";

class Game {
  constructor(board, url) {
    this.board = board;
    if (url !== undefined) {
      this.socket = new WebSocket(url);
    } else {
      this.socket = new WebSocket("ws://localhost:8080/games/ws/");
    }
    this.socket.reference = this;
    this.create_modal();
    this.socket.onopen = this.onopen;
    this.socket.onclose = this.onclose;
  }

  create_modal() {
    this.loading = {
      modal: undefined,
      content: undefined,
      progress: undefined,
    };

    // bind bs modal
    this.loading.modal = new bootstrap.Modal(
      document.getElementById("loading-modal"),
      {
        show: true,
        backdrop: "static",
        keyboard: false,
        focus: true,
      }
    );

    // bind modal-title and modal-content
    this.loading.content = document.getElementById("modal-content");

    // set base progress
    this.loading.progress = document.getElementById("connection-progress");
    this.loading.progress.style.width = "0";
    this.loading.progress.setAttribute("aria-valuenow", "0");

    // show modal
    this.loading.modal.show();
  }

  onopen(event) {
    console.log(this.reference);
    // set new progress
    this.reference.loading.content.innerHTML = "Connected to Websocket";
    this.reference.loading.progress.style.width = "25";
    this.reference.loading.progress.setAttribute("aria-valuenow", "25");

    this.send(JSON.stringify({ action: 1, data: { key: "val" } }));
  }

  onclose(event) {
    this.reference.loading.content.innerHTML =
      "Websocket Closed by server. Are you connected to the internet?";
    this.reference.loading.progress.style.backgroundColor = "red";
    this.reference.loading.progress.style.width = "100";
    this.reference.loading.progress.setAttribute("aria-valuenow", "100");
  }
}

document.addEventListener("DOMContentLoaded", function () {
  /*
   Check for screen size and show popup if incompatible
  */

  if (screen.height < 600 || screen.width < 1000) {
    // Create the element using the create_alertScreen Size method.
    create_alert(
      "Compatibilty",
      "danger",
      "Your screen is too small for this game."
    );
  }

  /*
    Bind modal
  */

  // draw the initial board
  const size = 1000;

  let drawer = SVG().addTo("#penta");
  drawer.addClass("allow-overflow responsive-img");
  drawer.viewbox(0, 0, size, size);
  drawer.attr({
    preserveAspectRatio: "xMidYMid meet",
    id: "penta",
  });

  drawer.data({ size: size });
  const math = new PentaMath(drawer);
  // I would like to have some sort of collapse where you could toggle
  // e.g. 'shift' or colors.
  let board = math.draw(drawer, size, { shift: true });

  /*
   This doesn't do authentication as the request is handled with SessionCookies
   */
  const instance = new Game(board);

  Array.prototype.slice
    .call(document.querySelectorAll("[data-id]"))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
