import {
  /* webpackMode: "eager" */
  Junction,
  Corner,
  COLORS,
} from "./core.js";
import { PentaMath } from "./pentagame.js";
import {
  /* webpackMode: "eager" */
  getJSONP,
  download,
  create_alert,
} from "./utils.js";

const SCALE = 1000;

class Game {
  constructor(url) {
    this.url = url;
  }

  create_modal() {
    this.loading = {
      modal: undefined,
      content: undefined,
      progress: undefined,
    };

    // bind bs modal
    this.loading.modal_el = document.getElementById("loading-modal");
    this.loading.modal = new bootstrap.Modal(this.loading.modal_el, {
      show: true,
      backdrop: "static",
      keyboard: false,
      focus: true,
    });

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

    this.drawer.data({ size: SCALE });
    this.math = new PentaMath(this.drawer);
    this.board = this.math.draw(SCALE);
  }

  open() {
    // start drawing
    // those are done first as they don't rely on external data for creation
    this.draw_board();
    this.create_modal();

    // INFO: Change for production
    if (this.url !== undefined) {
      this.socket = new WebSocket(this.url);
    } else {
      this.socket = new WebSocket("ws://localhost:8080/games/ws/");
    }

    this.socket.reference = this;
    this.socket.onopen = this.onopen;
    this.socket.onclose = this.onclose;
    this.socket.onmessage = this.onmessage;
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
    // save meta (includes users)
    this.meta = meta;

    // update users
    this.update_users(this.meta.players);

    // set state
    let state = document.getElementById("game-state");
    /*
    Evaluate State description
    see server/db/model for mapping
    */
    meta.state = Number(meta.state);
    if (meta.state == 0) {
      state.innerHTML = "Game is not running";
    } else if (meta.state < 6) {
      state.innerHTML = `Waiting for move of player ${
        this.meta.players[meta.state]
      }`;
    } else if (meta.state < 11) {
      state.innerHTML = `Waiting for stopper placement of player ${
        this.meta.players[meta.state]
      }`;
    } else if (meta.state != 11) {
      let winners = "",
        winner_amount = meta.state - 10;
      for (let i = 0; i < winner_amount; i++) {
        if (i + 2 == winner_amount) {
          winners.concat(`${this.meta.players[i]} and `);
        } else {
          winners.concat(`${this.meta.players[i]}, `);
        }
      }
      state.innerHTML = `Game won by ${winners} Congratulations!`;
    }

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

const instance = new Game("ws://localhost:8080/games/ws/");

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
   This doesn't do authentication as the request is handled with SessionCookies
   */

  instance.open();
  globalThis.instance = instance;

  Array.prototype.slice
    .call(document.querySelectorAll("[data-id]"))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
