document.addEventListener('DOMContentLoaded', () => {
  var toastElList = [].slice.call(document.querySelectorAll('.toast'));
  var toastList = toastElList.map(function (toastEl) {
    let el = new bootstrap.Toast(toastEl, { autohide: false });

    el.show();

    return el;
  });
});
