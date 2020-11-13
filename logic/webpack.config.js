const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const dist = path.resolve(__dirname, '../server/static/wasm-js');

module.exports = {
  mode: 'production',
  entry: {
    index: './js/main.js'
  },
  output: {
    path: dist,
    filename: '[name].js'
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, 'static')]),

    new WasmPackPlugin({
      crateDirectory: __dirname
    })
  ]
};
