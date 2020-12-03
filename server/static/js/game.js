import {
  /* webpackMode: "eager" */
  Junction,
  Corner,
  COLORS,
  SlimPentaMath,
} from "./core.js";
import {
  /* webpackMode: "eager" */
  getJSONP,
  download,
  create_alert,
} from "./utils.js";

class Game {
  constructor(scale, url) {
    this.board = {
      junctions: [],
      corners: [],
      stops: [],
      scale: scale,
    };

    // INFO: Change for production
    if (url !== undefined) {
      this.socket = new WebSocket(url);
    } else {
      this.socket = new WebSocket("ws://localhost:8080/games/ws/");
    }

    this.draw_board();
    this.create_modal();

    this.socket.reference = this;
    this.socket.onopen = this.onopen;
    this.socket.onclose = this.onclose;
    this.socket.onmessage = this.onmessage;
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

  draw_board() {
    this.drawer = SVG().addTo("#penta");
    this.drawer.addClass("allow-overflow responsive-img");
    this.drawer.attr({
      preserveAspectRatio: "xMidYMid meet",
      id: "penta",
    });

    this.drawer.data({ size: this.board.scale });
    const math = new SlimPentaMath(this.board.scale);

    this.board.bg_circle = this.drawer.circle(
      math.schrinked_scale + math.lw * 5
    );
    this.board.bg_circle.attr({
      cx: math.center.x,
      cy: math.center.y,
      fill: COLORS.background,
      id: "background-circle",
    });

    this.board.outer_bg_circle = drawer.circle(math.outer_circle * 2);
    this.board.outer_bg_circle.attr({
      cx: math.center.x,
      cy: math.center.y,
      fill: "none",
      stroke: colors.foreground,
      "stroke-width": lineWidth,
    });
    OuterBGCircle.data({ id: "outer-circle" });

    for (let i = 0; i < 5; i++) {
      this.board.junctions.push(new Junction(math).draw(this.drawer, i));
      this.board.corners.push(new Corner(math).draw(this.drawer, i));
    }
  }

  onopen(event) {
    console.log(this.reference);
    // set new progress
    this.reference.loading.content.innerHTML = "Connected to Websocket";
    this.reference.loading.progress.style.width = "25";
    this.reference.loading.progress.setAttribute("aria-valuenow", "25");

    // startup trigger
    this.reference.startup();
  }

  startup() {
    // start intial setup
    this.socket.send(JSON.stringify({ action: 0, data: {} }));
    this.socket.send(JSON.stringify({ action: 1, data: {} }));

    // finish
    this.loading.modal.hide();
  }

  update_users(users) {
    this.users = users;
    let list = document.getElementById("game-players");
    list.innerHTML = "";
    let user;
    for (let i = users.length - 1; i > -1; i--) {
      user = users[i];
      let item = document.createElement("li");
      item.classList.add(
        "list-group-item",
        "d-flex",
        "justify-content-between",
        "align-items-center"
      );
      console.log(`${user}`);
      item.innerHTML = `${user[1]} <span class="badge bg-primary rounded-pill">?</span>`;
      list.appendChild(item);
    }

    this.loading.progress.classList.remove("w-50");
    this.loading.content.innerHTML = "Loaded Users";
    this.loading.progress.style.width = "65%";
    this.loading.progress.setAttribute("aria-valuenow", "65");
  }

  update_metadata(meta) {
    // save meta
    this.meta = meta;

    // set state
    let state = document.getElementById("game-state");
    state.innerHTML = meta.state;

    // set description
    let description = document.getElementById("game-description");
    description.innerHTML = meta.description;

    // set name
    let name = document.getElementById("game-name");
    name.innerHTML = meta.name;

    this.loading.content.innerHTML = "Loaded Metadata";
    this.loading.progress.style.width = "75%";
    this.loading.progress.setAttribute("aria-valuenow", "75");
  }

  update_board(data) {
    // test
    this.board.drawFigure(data.figure);
  }

  onmessage(event) {
    let data = JSON.parse(event.data);
    switch (data.action) {
      case 0:
        // comes as (Uuid, String)
        this.reference.update_users(data.data);
        break;

      case 1:
        this.reference.update_metadata(data.data);
        break;

      case 2:
        this.reference.update_board(data.data);
        break;
      default:
        console.debug(data);
        create_alert(
          "Protocol Error",
          "danger",
          "Server responded with an unknown action code"
        );
        break;
    }
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

  /*
   This doesn't do authentication as the request is handled with SessionCookies
   */
  const instance = new Game(size);

  Array.prototype.slice
    .call(document.querySelectorAll("[data-id]"))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
