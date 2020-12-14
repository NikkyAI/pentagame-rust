import {
  /* webpackMode: "eager" */
  destructureID,
} from "./utils.js";

const COLORS = {
  fields: ["blue", "white", "green", "yellow", "red"],
  background: "#28292b",
  foreground: "#d3d3d3",
};

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

export { COLORS, Figure, Point };
