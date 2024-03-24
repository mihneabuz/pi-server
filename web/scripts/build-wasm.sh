#!/bin/bash

apps="$(pwd)/public/wasm"

rm -r "${apps}"
mkdir -p "${apps}"

for wasmapp in wasm/*; do
  pushd "${wasmapp}"

  wasm-pack build --release --target web

  cp -r pkg/*.wasm pkg/*.js "${apps}"
done
