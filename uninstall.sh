#!/bin/sh
set -eu

# Get env
PREFIX="${PREFIX:-/usr/local}"
INSTALLPATH="$PREFIX/bin/restic-ez"

# Uninstall the binary if it exists
if test -f "$INSTALLPATH"; then
    rm -v "$INSTALLPATH"
fi