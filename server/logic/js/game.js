import { PentaMath } from "./pentagame.js";
import { SVG } from "@svgdotjs/svg.js";
import { getJSONP, download } from "./core.js";

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
  // I would like to have some sort of collapse where you could toggle
  // e.g. 'shift' or colors.
  let board = math.draw(drawer, size, { shift: true });

  Array.prototype.slice
    .call(document.querySelectorAll("[data-id]"))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
