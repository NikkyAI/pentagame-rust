const path = require('path');

const dist = path.resolve(__dirname, './dist/');

/*
SASS/ PurgeCSS is not handled here, because there have been some problems with setting it up consistently
If you know webpack and wanna give this a try feel free to open a pull request
*/

module.exports = {
  mode: 'production', // change to 'development' when you need to debug in browser
  entry: {
    main: './js/main.js'
  },
  output: {
    path: dist,
    publicPath: '/static/js/',
    filename: '[name].js'
  }
};
