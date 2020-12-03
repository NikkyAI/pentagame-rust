"use strict";

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
      throw new TypeError("param " + i + " must be of type " + types[i]);
    }
  }
}

// function to create an alert before the body with a given level
function create_alert(alert, level, message) {
  let new_alert = document.createElement("div");
  new_alert.classList.add(
    "sticky-top",
    "alert",
    "mb-1",
    "alert-dismissible",
    `alert-${level}`
  );
  new_alert.setAttribute("role", "alert");
  new_alert.innerHTML = `<strong>${alert}</strong>  ${message}  <button type="button" class="btn-close" data-dismiss="alert" aria-label="Close"></button>`;
  document.getElementById("content").prepend(new_alert);
}

// can trigger download for abitary data
function download(filename, text) {
  var element = document.createElement("a");
  element.setAttribute(
    "href",
    "data:text/plain;charset=utf-8," + encodeURIComponent(text)
  );
  element.setAttribute("download", filename);

  element.style.display = "none";
  document.body.appendChild(element);

  element.click();

  document.body.removeChild(element);
}

// download data with a Promise for e.g. API reequests and parse with JSON
let getJSONP = (obj) => {
  return new Promise((resolve, reject) => {
    let xhr = new XMLHttpRequest();

    // open connection
    if (obj.method === "POST") {
      xhr.open("POST", obj.url);
    } else {
      xhr.open("GET", obj.url);
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
  if (typeOf(id) === "number") {
    return id;
  }
  id = id.split("-");
  const type = id[0];
  if (["corner", "junction", "c", "j"].include(type)) {
    return {
      type: type,
      id: id[1],
    };
  } else if (type === "stop" || type === "s") {
    return {
      type: stop,
      start: id[1],
      counter: id[2],
      end: id[3],
    };
  }
};

export { getJSONP, download, create_alert, destructureID };
