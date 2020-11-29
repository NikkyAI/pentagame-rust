const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const dist = path.resolve(__dirname, '../static/dist/');

module.exports = {
  mode: 'development', // change to 'development' when you need to debug in browser
  entry: {
    'index-logic': './js/main.js',
    game: './js/game.js',
    settings: './js/settings.js',
    'game-join': './js/game-join.js'
  },
  output: {
    path: dist,
    publicPath: '/static/js/',
    filename: '[name].js'
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname
    })
  ]
};
