#!/bin/bash
set -euxo pipefail

cd "$(dirname "$(readlink -f "$0")")"

TARGET='wasm32-unknown-unknown'
cargo build --target "$TARGET"
wasm-bindgen --out-dir ./target/webroot --target web "./target/$TARGET"/debug/*.wasm
cp -a ./*/websrc/* ./target/webroot/
