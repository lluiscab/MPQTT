#!/bin/bash
# aarch64-unknown-linux-gnu

echo "Installing cargo-deb"
cargo install cargo-deb

echo "Packaging binary into .deb file"
cargo deb --target aarch64-unknown-linux-gnu