'use strict';

/* eslint-disable no-undef */
const typeOf = (obj) => {
  return {}.toString
    .call(obj)
    .match(/\s(\w+)/)[1]
    .toLowerCase();
};

function checkTypes(args, types) {
  args = [].slice.call(args);
  for (var i = 0; i < types.length; ++i) {
    if (typeOf(args[i]) != types[i]) {
      throw new TypeError('param ' + i + ' must be of type ' + types[i]);
    }
  }
}

// function to create an alert
function create_alert(level, message) {
  let icon, title;
  // evaluate level
  switch (level) {
    case 0:
      icon = 'exclamation-triangle';
      title = 'DANGER';
      level = 'info';
      break;
    case 2:
      icon = 'exclamation-circle';
      title = 'Warning';
      level = 'warning';
      break;
    case 1:
      icon = 'info-circle';
      title = 'Notification';
      level = 'info';
      break;
  }

  // create new element and add classes see base.html for reference
  let toast_el = document.createElement('div');
  toast_el.classList.add('toast', 'mt-4', `bg-${level}`, 'text-white');

  // set role and aria attributed for support
  toast_el.setAttribute('role', 'alert');
  toast_el.setAttribute('aria-atomic', 'true');
  toast_el.setAttribute('aria-live', 'assertive');

  // add subheader
  let toast_header_el = document.createElement('div');
  toast_header_el.classList.add('toast-header', `bg-${level}`);

  // create & add img to header
  let toast_header_img_el = document.createElement('i');
  toast_header_img_el.setAttribute('alt', `alert ${level} icon`);
  toast_header_img_el.classList.add('mr-2', 'text-white', 'fas', `fa-${icon}`);

  toast_header_el.appendChild(toast_header_img_el);

  // create title
  let toaster_head_title = document.createElement('strong');
  toaster_head_title.innerHTML = title;
  toaster_head_title.classList.add('title-light', 'mr-auto', 'text-white');

  toast_header_el.appendChild(toaster_head_title);

  // add close button
  let toast_header_close_button = document.createElement('button');
  toast_header_close_button.setAttribute('type', 'button');
  toast_header_close_button.setAttribute('data-dismiss', 'toast');
  toast_header_close_button.setAttribute('aria-label', 'Close');
  toast_header_close_button.classList.add('btn-close', `bg-${level}`);

  toast_header_el.appendChild(toast_header_close_button);

  // add header
  toast_el.appendChild(toast_header_el);

  // add message
  let toast_message_el = document.createElement('div');
  toast_message_el.classList.add('toast-body');
  toast_message_el.innerHTML = message;

  toast_el.appendChild(toast_message_el);

  // add to toast container
  document.getElementById('toast-container').appendChild(toast_el);
  let el = new bootstrap.Toast(toast_el, { autohide: false });
  el.show();
}

// can trigger download for abitary data
function download(filename, text) {
  var element = document.createElement('a');
  element.setAttribute(
    'href',
    'data:text/plain;charset=utf-8,' + encodeURIComponent(text)
  );
  element.setAttribute('download', filename);

  element.style.display = 'none';
  document.body.appendChild(element);

  element.click();

  document.body.removeChild(element);
}

// download data with a Promise for e.g. API reequests and parse with JSON
let getJSONP = (obj) => {
  return new Promise((resolve, reject) => {
    let xhr = new XMLHttpRequest();

    // open connection
    if (obj.method === 'POST') {
      xhr.open('POST', obj.url);
    } else {
      xhr.open('GET', obj.url);
    }

    if (obj.headers) {
      Object.keys(obj.headers).forEach((key) => {
        xhr.setRequestHeader(key, obj.headers[key]);
      });
    }

    xhr.onload = () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve(JSON.parse(xhr.response));
      } else {
        reject(xhr.statusText);
      }
    };

    xhr.onerror = () => reject(xhr.statusText);
    xhr.send(obj.body);
  });
};

// destructure ID provided for field
const destructureID = (id) => {
  if (typeOf(id) === 'number') {
    return id;
  }
  id = id.split('-');
  const type = id[0];
  if (['corner', 'junction', 'c', 'j'].include(type)) {
    return {
      type: type,
      id: id[1]
    };
  } else if (type === 'stop' || type === 's') {
    return {
      type: stop,
      start: id[1],
      counter: id[2],
      end: id[3]
    };
  }
};

export { getJSONP, download, create_alert, destructureID };
