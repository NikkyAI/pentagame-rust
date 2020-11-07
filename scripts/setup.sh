#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Setup script for pentagame
# Supports yarn and npm
# under GPL v3.0 @ Cobalt <cobalt.rocks> (see LICENSE)

# Library directory
LIB_DIR="$PWD/static/"

# Determine if yarn and/ or npm is available and exit with error otherwise
if command -v yarn &>/dev/null; then
    TOOL="yarn install -s"
elif command -v npm &>/dev/null; then
    TOOL="npm install --silent -f"
else
    echo "You need either yarn or npm to use this script"
    exit 1
fi

echo "Using '$TOOL' to install third-party libraries"

# Install with yarn and/ or npm
cd "$LIB_DIR"
$TOOL

echo "Done. Thank you for using pentagame"
