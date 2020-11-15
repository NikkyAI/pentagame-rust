/*
Pentagame.js borrowed from https://github.com/penta-jan/pentagames
under unknown @ Pentajan
*/

// Hier beginnt die Pentagame Programmiererei...

var canvas_click;
var canvas_board;
var canvas_play;
var context_click;
var context_board;
var context_play;
var pentaGame;
var sqrt5 = Math.sqrt(5);
var phi = (sqrt5 + 1) / 2;
var rs = 12; // radius step (litle gray dots)  // this is the only absolute variable
var ros = (8 / 12) * rs; // radius outer rim step // utter bollocks
var rj = (rs * (9 - 2 * sqrt5)) / sqrt5; // 19;  // radius junction
var rc = rs * sqrt5; //	21;  // radius corner
var rr = (2 * rc + 4 * rj + 30 * rs) / Math.sqrt(phi + 2); // radius mid corner
var rR = rr + rc / 2; // outer radius
var centerX = rR; // 250;
var centerY = rR; // 250;

// debug helper
function log(message) {
  if (window.console) {
    console.log(message);
  }
}

function setupPentaGame() {
  canvas_click = document.getElementById('pentacanvas_click');

  // Make sure we don't execute when canvas isn't supported
  if (canvas_click.getContext) {
    context_click = canvas_click.getContext('2d');
  } else {
    return false;
  }
  canvas_board = document.getElementById('pentacanvas_board');
  context_board = canvas_board.getContext('2d');
  canvas_play = document.getElementById('pentacanvas_play');
  context_play = canvas_play.getContext('2d');
  //canvas_play.style.cursor = "pointer, auto";
  //document.getElementById('container').style.cursor = 'crosshair';
  //document.getElementById('container').style.cursor = 'help';
  return true;
}

function drawCircle(context, X, Y, radius, colour) {
  context.beginPath();
  context.arc(X, Y, radius, 0, 2 * Math.PI, false);
  context.fillStyle = colour;
  context.fill();
  context.lineWidth = 1;
  context.strokeStyle = '#003300';
  context.stroke();
}

function drawNumber(context, x, y, bgColor, number) {
  var col = 'white';
  switch (bgColor) {
    case 'white':
    case 'yellow':
      col = 'black';
  }
  context.fillStyle = col;
  context.fillText(number + '.', x - 6, y + 4);
}

var magic_map = {};

function addCircle(context, X, Y, radius, isBoard, colour, r, g, b) {
  if (isBoard) {
    drawCircle(context, X, Y, radius, colour);
  } else {
    var i = rgb(r, g, b);
    drawCircle(context, X, Y, radius, i);
    magic_map[i] = { X: X, Y: Y, radius: radius };
  }
}

function getColourName(index) {
  switch (index) {
    case 0:
      return 'white';
    case 1:
      return 'blue';
    case 2:
      return 'red';
    case 3:
      return 'yellow';
    case 4:
      return 'green';
    default:
      alert('Not a valid colour index: ' + index);
  }
}

function rgb(r, g, b) {
  return 'rgb(' + r + ', ' + g + ', ' + b + ')';
}

function posString(r, g, b) {
  return '[' + r + ', ' + g + ', ' + b + ']';
}

function rotate(cx, cy, x, y, angle) {
  var radians = (Math.PI / 180) * angle,
    cos = Math.cos(radians),
    sin = Math.sin(radians),
    nx = cos * (x - cx) + sin * (y - cy) + cx,
    ny = cos * (y - cy) - sin * (x - cx) + cy;
  return [nx, ny];
}

function drawPentaGame(context, isBoard) {
  //
  // don't touch the next 3 assignments!!!!
  var numRimCirc = 13; // this many circles on the rim between startpoints
  var rimAngleInc = -(60 / numRimCirc); // heuristics rulez!!!
  var firstRot = -((Math.PI * rR) / 180) * (12 / numRimCirc);

  // magic constants
  var sinusfactor = Math.sin(Math.PI / 10); // 18 * Math.PI / 180
  var cosinusfactor = Math.cos(Math.PI / 10); // 18 * Math.PI / 180
  var rotatefactor = (72 * Math.PI) / 180;
  var ratio1 = rc + rj + 2 * rs * 6;

  if (isBoard) {
    // draw central circle
    drawCircle(context, centerX, centerY, rR + rs, 'gray');
    drawCircle(context, centerX, centerY, rj, 'silver');
  } else {
    context.beginPath();
    context.fillStyle = 'gray';
    context.fillRect(0, 0, canvas_click.width, canvas_click.height);
    context.stroke();
  }

  context.font = '18px sans-serif';
  for (i = 0; i < 5; i++) {
    // startpoint
    var spX = centerX + rr;
    var spY = centerY;
    var myColor = getColourName(i);
    addCircle(context, spX, spY, rc, isBoard, myColor, i, 0, 0);
    if (isBoard) {
      drawNumber(context, spX, spY, myColor, i);
    }

    var ii = i < 3 ? i + 2 : i - 3;
    //		var ii = (i);
    var ij = ii == 4 ? 5 : ii + 6;

    var ik = (i + 1) % 5; // needed for outer rim
    var rev = i > ik; // needed for rim, true between points 4 and 0

    for (j = 0; j < 6; j++) {
      // connect each startpoint with 2 neighbor junctions and startpoints
      var m1 = rc + rs + 2 * rs * j;
      addCircle(
        context,
        spX - cosinusfactor * m1,
        spY - sinusfactor * m1,
        rs,
        isBoard,
        'silver',
        i,
        ii + 5,
        j + 1
      );
      addCircle(
        context,
        spX - cosinusfactor * m1,
        spY + sinusfactor * m1,
        rs,
        isBoard,
        'silver',
        i,
        ij,
        j + 1
      );
    }

    var tempP = rotate(centerX, centerY, spX, spY, firstRot);
    var xx = tempP[0]; // follows a circle path (cacheable)
    var yy = tempP[1]; // xx and yy are overwritten every rot step

    for (j = 0; j < numRimCirc; j++) {
      tempP = rotate(centerX, centerY, xx, yy, rimAngleInc);
      xx = tempP[0];
      yy = tempP[1];
      addCircle(
        context,
        xx,
        yy,
        ros,
        isBoard,
        'silver',
        rev ? ik : i,
        rev ? i : ik,
        rev ? numRimCirc - j : j + 1
      );
    }
    var m2 = cosinusfactor * ratio1;
    var m3 = sinusfactor * ratio1;

    myColor = getColourName(ii);
    // draw junction
    addCircle(context, spX - m2, spY - m3, rj, isBoard, myColor, ii + 5, 0, 0);
    if (isBoard) {
      drawNumber(context, spX - m2, spY - m3, myColor, ii + 5);
    }

    for (j = 1; j <= 3; j++) {
      // connect junction spots
      addCircle(
        context,
        spX - m2,
        spY - m3 + rj + (j * 2 - 1) * rs,
        rs,
        isBoard,
        'silver',
        ii + 5,
        ij,
        j
      );
    }

    // Move registration point to the center of the canvas
    context.translate(centerX, centerY);
    // Rotate 72 degree
    context.rotate(rotatefactor);

    // Move registration point back to the top left corner of canvas
    context.translate(-centerX, -centerY);

    // rotate table coordinates
    var cos = Math.cos(rotatefactor);
    var sin = Math.sin(rotatefactor);

    for (key in magic_map) {
      var o = magic_map[key];
      var nx = cos * (o.X - centerX) + sin * (o.Y - centerY) + centerX;
      var ny = cos * (o.Y - centerY) - sin * (o.X - centerX) + centerY;
      o.X = nx;
      o.Y = ny;
      //log(magic_map[key]);
    }
  }
}

function clickHandler(event) {
  var totalOffsetX = 0;
  var totalOffsetY = 0;
  var x = 0;
  var y = 0;

  var currentElement = canvas_click;

  do {
    totalOffsetX += currentElement.offsetLeft - currentElement.scrollLeft;
    totalOffsetY += currentElement.offsetTop - currentElement.scrollTop;
  } while ((currentElement = currentElement.offsetParent));

  x = event.pageX - totalOffsetX;
  y = event.pageY - totalOffsetY;

  var c = context_click.getImageData(x, y, 1, 1);
  var i = rgb(c.data[0], c.data[1], c.data[2]);
  var j = posString(c.data[0], c.data[1], c.data[2]);

  var o = magic_map[i];

  if (!o) {
    pentaGame.log.log('x:' + x + ' y:' + y + ' Clicked somewhere');
  } else {
    pentaGame.log.log('x:' + x + ' y:' + y + ' ID:' + j + ' o:' + o);
    context_play.clearRect(0, 0, canvas_click.width, canvas_click.height);
    drawCircle(context_play, o.X, o.Y, o.radius, 'black');
  }

  //alert("x:" + x + " y:" + y + " R:" + c.data[0] + " G:" + c.data[1] + " B:" + c.data[2] + " A:" + c.data[3]);
  //log("x:" + x + " y:" + y + " R:" + c.data[0] + " G:" + c.data[1] + " B:" + c.data[2] + " A:" + c.data[3]);
  //log("x:" + x + " y:" + y + " ID:" + rgb(c.data[0], c.data[1], c.data[2]) + " A:" + c.data[3]);
  //log("x:" + x + " y:" + y + " ID:" + i + " o:" + o);

  //drawCircle(context_play, x, y, 5, 'black');
}

function doPentaGame() {
  setupPentaGame();
  pentaGame = new PentaGame();
  drawPentaGame(context_click, false);
  drawPentaGame(context_board, true);
  canvas_play.onclick = function (event) {
    clickHandler(event);
  };
  //canvas_click.style.cursor = 'pointer';
  canvas_play.style.cursor = 'cell'; // better visible than crosshair
}

function startPentaGame() {
  var startBtn = document.getElementById('startGame');
  startBtn.disabled = true;
  var stopBtn = document.getElementById('stopGame');
  stopBtn.disabled = false;
  var szSel = document.getElementById('spielerZahl');
  szSel.disabled = true;
  var fsSel = document.getElementById('farbSchema');
  fsSel.disabled = true;
  pentaGame.log.log('startPentaGame');
}

function stopPentaGame() {
  var startBtn = document.getElementById('startGame');
  startBtn.disabled = false;
  var stopBtn = document.getElementById('stopGame');
  stopBtn.disabled = true;
  var szSel = document.getElementById('spielerZahl');
  szSel.disabled = false;
  var fsSel = document.getElementById('farbSchema');
  fsSel.disabled = false;
  pentaGame.log.log('stopPentaGame');
}

class PentaGameLog {
  constructor() {}

  log(message) {
    var l = document.getElementById('gameLog');
    var t = document.createTextNode(message);
    var p = document.createElement('li');
    p.appendChild(t);
    l.appendChild(p);
  }

  sendToBrowser() {
    var msg = '';
    var l = document.getElementById('gameLog');
    var p = l.firstChild;
    //	msg += p.textContent;
    //	msg += "X";
    while (p.nextSibling) {
      p = p.nextSibling;
      msg += p.textContent;
      msg += '\n';
    }

    var dataStr = 'data:text/plain;charset=utf-8,' + encodeURIComponent(msg);
    var dlAnchorElem = document.getElementById('downloadAnchor');
    dlAnchorElem.setAttribute('href', dataStr);
    dlAnchorElem.setAttribute('download', 'pentagame.log');
    dlAnchorElem.click();
  }
}

class PentaGameBoard {
  constructor() {
    this._rotation = 0;
    this._position_map = {};
  }

  setRotation(value) {
    if (this._rotation == value) {
      return;
    }
    this._rotation = value;
    this.drawBoard();
  }

  drawBoard() {}
}

////////////////////////////
// the game itself
class PentaGame {
  constructor() {
    this.log = new PentaGameLog();
    this.board = new PentaGameBoard();
  }
}

////////////////////////////
// callbacks for ui controls

function onRotateBoard(element) {
  // sort out invalids
  if (!Number.isInteger(element.valueAsNumber)) {
    pentaGame.log.log('onChange: not an integer:' + element.value);
    return;
  }
  if (element.valueAsNumber < -1 || element.valueAsNumber > 360) {
    pentaGame.log.log('onChange: out of range:' + element.value);
    return;
  }
  // roll over
  if (element.value == 360) {
    element.value = 0;
  }
  if (element.value == -1) {
    element.value = 359;
  }

  // ok, set it
  pentaGame.board.setRotation(element.value);
}

// vim: ts=2 sw=2 tw=0 noet
