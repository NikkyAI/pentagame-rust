function create_modal(text, title, confirmation, callback) {
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
  modal_card.id = 'danger-card';
  modal_card.innerHTML = `<div class="card" style="width: 80%;"><div class="card-body bg-danger text-white"><h2 class="card-title title-white">${title}</h2><p class="card-text">${text}</p><div class="btn-group" role="group" aria-label="Dangerous Section button group"><button type="button" id="destroy-danger" class="btn btn-danger text-white"><i class="fas fa-times"></i> Close</button><button type="button" id="continue-danger" class="btn btn-danger text-white"><i class="fas fa-check"></i> ${confirmation}</button></div></div></div>`;
  // Finally, append the element to the HTML body
  document.body.appendChild(modal_card);

  document.getElementById('destroy-danger').addEventListener('click', () => {
    document.getElementById('danger-card').remove();
  });

  document
    .getElementById('continue-danger')
    .addEventListener('click', callback);
}

document.addEventListener('DOMContentLoaded', () => {
  document
    .getElementById('archive-account')
    .addEventListener('click', (event) => {
      event.preventDefault();
      create_modal(
        "This will make your account be shown as archived. You will not be able to log into this account anymore but you're game history will be kept.",
        'Archiving your Account',
        'Continue',
        () => {
          // send the data
          var xhr = new XMLHttpRequest();
          xhr.open('POST', '/users/settings/archive', true);

          xhr.send();
        }
      );
    });

  document
    .getElementById('delete-account')
    .addEventListener('click', (event) => {
      event.preventDefault();
      create_modal(
        "This will delete all data related to your account including your game history. THIS IS NOT REVERSIBLE. We won't be able to restore your account.",
        'Deleting your Account',
        'Confirm',
        () => {
          // send the data
          var xhr = new XMLHttpRequest();
          xhr.open('POST', '/users/settings/delete', true);

          xhr.send();
        }
      );
    });
});
