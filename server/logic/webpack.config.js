const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "../static/js/");

module.exports = {
  mode: "production", // change to 'development' when you need to debug in browser
  entry: {
    index: "./js/main.js",
    game: "./js/game.js",
    "game-join": "./js/game-join.js",
  },
  output: {
    path: dist,
    publicPath: "/static/js/",
    filename: "[name].js",
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};
