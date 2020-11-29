import { PentaMath } from './pentagame.js';
import { SVG } from '@svgdotjs/svg.js';
import { getJSONP, download } from './core.js';

document.addEventListener('DOMContentLoaded', function () {
  /*
   Check for screen size and show popup if incompatible
  */

  if (screen.height < 600 || screen.width < 1000) {
    //Create the element using the createElement method.
    var modal_card = document.createElement('card');
    modal_card.classList.add(
      'd-flex',
      'align-items-center',
      'justify-content-center'
    );
    modal_card.style.zIndex = 1100;
    modal_card.style.position = 'fixed';
    modal_card.style.top = '4rem';
    modal_card.style.width = '100%';
    modal_card.id = 'warning-card';
    modal_card.innerHTML =
      '<div class="card" style="width: 80%;"><div class="card-body bg-warning text-white"><h2 class="card-title title-white">Warning</h2><p class="card-text">Your screen is smaller than the required screen size. Consider flipping your phone/ tablet in horizontal mode or playing this game on a PC.</p><button type="button" id="destroy-warning" class="btn btn-warning text-white">Continue anyway</button></div></div>';
    // Finally, append the element to the HTML body
    document.body.appendChild(modal_card);

    document.getElementById('destroy-warning').addEventListener('click', () => {
      document.getElementById('warning-card').remove();
    });
  }

  /*
    Bind modal
  */
  var myModal = new bootstrap.Modal(document.getElementById('myModal'), {
    show: true,
    backdrop: 'static',
    keyboard: false,
    focus: true
  });
  myModal.show();
  let close = () => {
    myModal.hide();
  };

  // demo
  setTimeout(() => {
    let progress = document.getElementById('connection-progress');
    progress.classList.remove('w-50');
    progress.classList.add('w-75');
    progress.setAttribute('aria-valuenow', '75');
  }, 2000);
  setTimeout(() => {
    let progress = document.getElementById('connection-progress');
    progress.classList.remove('w-75');
    progress.classList.add('w-100');
    progress.setAttribute('aria-valuenow', '100');
  }, 3000);
  setTimeout(close, 3500);

  /*
   This doesn't do authentication as the request is handled with SessionCookies
   */
  let gsocket = new WebSocket('ws://localhost:8080/games/ws/');
  gsocket.onopen = (event) => {
    gsocket.send(JSON.stringify({ action: 1, data: { key: 'val' } }));
  };

  // draw the initial board
  const size = 1000;

  let drawer = SVG().addTo('#penta');
  drawer.addClass('allow-overflow responsive-img');
  drawer.viewbox(0, 0, size, size);
  drawer.attr({
    preserveAspectRatio: 'xMidYMid meet',
    id: 'penta'
  });

  drawer.data({ size: size });
  const math = new PentaMath(drawer);
  // I would like to have some sort of collapse where you could toggle
  // e.g. 'shift' or colors.
  let board = math.draw(drawer, size, { shift: true });

  Array.prototype.slice
    .call(document.querySelectorAll('[data-id]'))
    .map(function (element) {
      element.onclick = (evt) => {
        console.log(`You have clicked the element ${this.dataset.id}`);
      };
    });
});
