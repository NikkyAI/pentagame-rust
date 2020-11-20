#!/usr/bin/env bash
# -*- coding: utf-8 -*-
# Full setup script for pentagame
# by Cobalt <https://cobalt.rocks> under GPLv3.0-or-newer

# Ensuring all the programs are installed
# Check if rust is installed
if ! command -v cargo &>/dev/null; then
    echo "You need cargo to build the application"
    exit 1
fi

# Check if wasm-pack is installed and if not install it
if ! command -v wasm-pack; then
    echo "You need wasm-pack to build the logic libraries. (use: cargo install wasm-pack)"
    exit 1
fi

# Check if GNU make is installed
if ! command -v make &>/dev/null; then
    echo "You need GNU make to build pentagame."
    exit 1
fi

# Determine if yarn and/ or npm is available and exit with error otherwise
# WARNING: There's another utility also called yarn for some systems which could trigger this too
if command -v yarn &>/dev/null; then
    TOOL="yarn install -s"
elif command -v npm &>/dev/null; then
    TOOL="npm install --silent -f"
else
    echo "You need either yarn or npm to use this script"
    exit 1
fi

# Determine if diesel-cli & GNU grep are installed for db-setup
if ! command -v diesel &>/dev/null; then
    echo "You need the diesel cli. (use: cargo install diesel_cli --no-default-features --features postgres)"
    exit 1
elif ! command -v grep &>/dev/null; then
    echo "You need GNU grep for using this script"
    exit 1
fi

# Start setup
echo "Starting with setup"
echo "Descending into server/logic/ to build pentagame-logic library"
cd server/logic/

wasm-pack build
cd pkg/ && yarn link && cd ../../static/ && yarn link "pentagame-logic" && cd ../../ # Npm is always installed if yarn is installed

echo "Done building. Ascending back"

echo "Descending into server and start building"

cd server/
make setup

if ! test -f "pentagame.toml"; then
    echo "You need to configure pentagame.toml in server/ before building any further"
    exit 1
fi

make db-setup generate build

echo "Done building server. Call 'make serve' in server/ to start instance"
