#!/bin/sh
set -eu

# Get env
PREFIX="${PREFIX:-/usr/local/bin}"

# Build and install the application
cargo install --path "./" --root "$PREFIX/"
