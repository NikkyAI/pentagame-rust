#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Setup script for pentagame
# Supports yarn and npm
# under GPL v3.0 @ Cobalt <cobalt.rocks> (see LICENSE)

# Library directory
LIB_DIR="$PWD/static/"

echo "Using '$TOOL' to install third-party libraries"

# Install with yarn and/ or npm
cd "$LIB_DIR"
$TOOL
cd ../

echo "Done. Thank you for using pentagame"
