import { PentaMath } from "./pentagame.js";
import { SVG } from "@svgdotjs/svg.js";
import { getJSONP, download } from "./core.js";
let shift = false;
let data = null;

document.addEventListener("DOMContentLoaded", function () {
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
  let board = math.draw(drawer, size);
  /*
  // action btns
  const capture_btn = document.getElementById("btn-capture");
  capture_btn.onclick = (evt) => {
    download("pentagame.svg", drawer.svg());
  };

  const shift_btn = document.getElementById("btn-shift");
  shift_btn.onclick = (evt) => {
    while (drawer.node.lastChild) {
      drawer.node.removeChild(drawer.node.lastChild);
    }
    if (shift === true) {
      let board = math.draw(drawer, size);
      shift = false;
    } else {
      let board = math.draw(drawer, size, { shift: false });
      shift = true;
    }
  };

  const export_btn = document.getElementById("btn-export");
  export_btn.onclick = (evt) => {
    if (data !== null) {
      download("pentagame.json", JSON.stringify(data));
    } else {
      console.log("Data seems not loaded");
    }
  };

  const import_modal = document.getElementById("modal-import-form");
  import_modal.onsubmit = (evt) => {
    evt.preventDefault();
    var file = document.getElementById("modal-file-input").files;
    var reader = new FileReader();
    reader.onload = function (evt) {
      data = JSON.parse(evt.target.result);
      console.log("Data updated!");
    };
    reader.readAsText(file[0]);
  };

  var modals = document.querySelectorAll(".modal");
  var instances = M.Modal.init(modals);

*/
  Array.prototype.slice
    .call(document.querySelectorAll("[data-id]"))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
