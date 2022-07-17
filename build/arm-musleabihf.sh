#!/bin/bash
# https://github.com/mmstick/cargo-deb/issues/99

echo "Installing cargo-deb"
cargo install cargo-deb

echo Building binary on docker
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf cargo build --target arm-unknown-linux-musleabihf --release --features build-for-deb

echo "Fixing permissions"
sudo chmod 666 ./target/arm-unknown-linux-musleabihf/release/mpqtt

echo "Packaging binary into .deb file"
cargo deb --target arm-unknown-linux-musleabihf --no-build