#!/bin/bash

apps="$(pwd)/public/wasm"

for wasmapp in wasm/*; do
  pushd $wasmapp

  wasm-pack build --release --target web

  mkdir -p "${apps}"
  cp -r pkg/* "${apps}"
done
