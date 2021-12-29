#!/bin/bash
set -euxo pipefail

cd "$(dirname "$(readlink -f "$0")")"

TARGET='wasm32-unknown-unknown'
cargo build --target "$TARGET"

rm -r ./target/webroot

for wasm in "./target/$TARGET"/debug/*.wasm
do
  name="$(basename "$wasm" | sed 's/\.wasm$//')"

  wasm-bindgen \
    --target web \
    --out-dir "./target/webroot/$name" \
    "$wasm"

  cp -av ./$name/websrc/* ./target/webroot/$name
done

cd ./target/webroot
python3 -m http.server
