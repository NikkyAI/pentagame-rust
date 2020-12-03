import {
  /* webpackMode: "eager" */
  destructureID,
} from "./utils.js";

const COLORS = {
  fields: ["blue", "white", "green", "yellow", "red"],
  background: "#28292b",
  foreground: "#d3d3d3",
};

function helper(centerX, centerY, radius, angle, options) {
  if (options !== undefined && "shift" in options && options.shift === true) {
    angle = (angle * Math.PI) / 180 + (Math.PI / 180.0) * -18;
  } else {
    angle = (angle * Math.PI) / 180;
  }

  return {
    x: centerX + radius * Math.cos(angle),
    y: centerY + radius * Math.sin(angle),
  };
}

class SlimPentaMath {
  constructor(scale) {
    // holds the numerical constants
    this._constants = {
      l: 6, // legs
      k: 3, // arms
      p: Math.sqrt((25 - 11 * Math.sqrt(5)) / (5 - Math.sqrt(5))), // inner
      golden: (Math.sqrt(5) + 1) / 2, // golden section value
      theta: 18, // theta value
    };
    // holds the relative numerical relative values centered on s
    this._sizes = {
      s: 1, // stop on star
      c: Math.sqrt(5), // corner stop
      j: (9 - 2 * Math.sqrt(5)) / Math.sqrt(5), // junction stop
      r: (2 / 5) * Math.sqrt(1570 + 698 * Math.sqrt(5)), // pentagram (diameter)
    };
    this._sizes.R = this._sizes.r + this._sizes.c; // entire board
    this._sizes.outer_circle = (this._sizes.r / this._sizes.R) * 0.2; // background stroke width
    this._sizes.inner_r =
      ((this._constants.k + this._sizes.j) * (1.0 + this._sizes.c)) /
      Math.sqrt(2.0 * (5.0 + this._sizes.c));
    this._constants.sizes = this._sizes;
    this.constants = this._constants;

    // holds the numerical constants
    this._constants = {
      l: 6, // legs
      k: 3, // arms
      p: Math.sqrt((25 - 11 * Math.sqrt(5)) / (5 - Math.sqrt(5))), // inner
      golden: (Math.sqrt(5) + 1) / 2, // golden section value
      theta: 18, // theta value
    };
    // holds the relative numerical relative values centered on s
    this._sizes = {
      s: 1, // stop on star
      c: Math.sqrt(5), // corner stop
      j: (9 - 2 * Math.sqrt(5)) / Math.sqrt(5), // junction stop
      r: (2 / 5) * Math.sqrt(1570 + 698 * Math.sqrt(5)), // pentagram (diameter)
    };
    this._sizes.R = this._sizes.r + this._sizes.c; // entire board
    this._sizes.outer_circle = (this._sizes.r / this._sizes.R) * 0.2; // background stroke width
    this._sizes.inner_r =
      ((this._constants.k + this._sizes.j) * (1.0 + this._sizes.c)) /
      Math.sqrt(2.0 * (5.0 + this._sizes.c));
    this._constants.sizes = this._sizes;
    this.constants = this._constants;

    // evaluate basic points and values
    this.center = { x: 0.5 * scale, y: 0.5 * scale };
    this.scale = scale;
    this.schrinked_scale = scale * 0.8;
    this.lw = (0.1 / this.constants.sizes.R) * this.schrinked_scale;
    this.inner_radius =
      (this.schrinked_scale / this.constants.sizes.R) *
      this.constants.sizes.inner_r;
    this.outer_radius =
      this.schrinked_scale / this.constants.sizes.c + this.lw * 3.5;
    this.junction_radius =
      (this.schrinked_scale / this.constants.sizes.R) * this.constants.sizes.j;
    this.corner_radius =
      (this.schrinked_scale / this.constants.sizes.R) * this.constants.sizes.c;
    this.stop_radius =
      (this.schrinked_scale / this.constants.sizes.R) * this.constants.sizes.s;
  }
}

class Base {
  constructor(data) {
    /*
        Object representing a figure/ Field on the board
        */
    for ([key, val] in Object.entries(data)) {
      this[key] = val;
    }
  }

  calcPos(args) {
    /*
        function for calculating pos based on data given by constructor
        Must return an array with two numbers (int/ float)
        */
    const points = args; //
    return points;
  }

  getAdjacent() {
    /*
        function for getting adjacent Figures/ Fields
        Must return an array with their respective objects
        */
    return this.adjacent;
  }
}

class Figure extends Base {
  /*
    Class representing a Figure (Gray and black stoppers, Players) on the baord
    */

  constructor(data) {
    super(data);
    this.state.position = state;
    this.id = data.id;
    this.color = data.color;
    this.board = data.board;
  }

  setState(state) {
    this.state = state;
  }

  move(data) {
    console.log(data);
    return true;
  }
}

class Point {
  /*
     class representing a point in the coordinate system
     May contain explicit data (id, points) or inexplicit additional data (state)
    */
  constructor(data) {
    this.id = data.id;
    this.additional = {};
    for (const key of Object.keys(data)) {
      this.additional[key] = data[key];
    }
    this.points = { x: data.x, y: data.y };
  }
}

class Stop extends Base {
  constructor(data) {
    super(data);
    this.id = destructureID(data.id);
    this.points = data.points;
  }

  isEmpty(args) {
    for (const _val in Object.values(args.board.figures)) {
      for (const figure in _val) {
        if (figure.position.id === this.id) {
          if (args.return === true) {
            return figure;
          } else {
            return false;
          }
        }
      }
    }
    return true;
  }

  getAdjacent() {
    if (
      (this.id.start >= 6 && this.id.end < 6) ||
      (this.id.start < 6 && this.id.end >= 6)
    ) {
      return [];
    } else if (this.counter === 1 || this.id.counter === 3) {
      return [];
    } else {
      return [];
    }
  }
}

class Junction {
  constructor(math) {
    this.math = math;
  }

  draw(drawer, index) {
    this.id = this.index = index;
    this.color = COLORS.fields[index];
    this.node = drawer.circle(this.math.junction_radius);
    console.log(this.node);
    this.node.attr({
      fill: COLORS.foreground,
      stroke: this.color,
      "stroke-width": this.math.lw * 0.75,
    });
    this.angle = index * -72 + 180;

    let points = helper(
      this.math.center.x,
      this.math.center.y,
      this.math.inner_radius,
      this.angle,
      this.shift
    );
    this.point = new Point({
      x: points.x,
      y: points.y,
      next: this.id != 4 ? this.id + 1 : null,
      angle: this.angle,
    });

    console.log(this.node);
    this.node.center(points.x, points.y);
    this.node.data({ id: index + 1 });
  }
}

class Corner {
  constructor(math) {
    this.math = math;
    console.log(math);
  }

  draw(drawer, index) {
    this.id = this.index = index + 5;
    this.color = COLORS.fields[index];
    this.node = drawer.circle(this.math.corner_radius);
    this.node.attr({
      fill: COLORS.foreground,
      stroke: this.color,
      "stroke-width": this.math.lw * 0.75,
    });
    this.angle = index * -72;

    let points = helper(
      this.math.center.x,
      this.math.center.y,
      this.math.inner_radius,
      this.angle,
      { shift: true }
    );
    this.point = new Point({
      x: points.x,
      y: points.y,
      next: this.id != 9 ? this.id + 1 : null,
      angle: this.angle,
    });

    this.node.center(points.x, points.y);
    this.node.data({ id: this.id });
  }
}

export { Stop, Corner, Junction, COLORS, SlimPentaMath };
