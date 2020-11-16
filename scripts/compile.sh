#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Script for compiling frontend dependencies for pentagame
# Requires being run from yarn or with purgecss, sass (dart) and terser being globally installed
# under GPL v3.0 @ Cobalt <cobalt.rocks> (see LICENSE)

echo "Compiling SCSS -> CSS"
sass --style=compressed --load-path scss/ --load-path node_modules/ scss/main.scss css/app.css
purgecss --config purgecss.config.js --css css/app.css --content "../templates/**/*.html" --output css/

echo "Building WASM & Compiling JS"
cd ../logic/
npm run build
cd ../static/

echo "Done"
