#!/bin/sh

set -ex

pushd src/input
cargo test -v
popd

cargo build -v
cargo doc -v
