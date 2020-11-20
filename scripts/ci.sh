#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Script for preparing environment for build with github CI
# under GPL v3.0 @ Cobalt <cobalt.rocks> (see LICENSE)


# Copy ci spcific pentagame.toml to server/
cp .github/ci/ci.toml server/pentagame.toml
