#!/bin/sh
set -eu

# Get env
PREFIX="${PREFIX:-/usr/local}"
INSTALLPATH="$PREFIX/bin/restic-ez"

# Build and install
cargo build --release
install -v -m 0755 "target/release/restic-ez" "$INSTALLPATH"