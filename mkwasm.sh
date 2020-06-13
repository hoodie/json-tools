#!/usr/bin/bash

cargo wasi build --release
cp target/wasm32-wasi/release/*.wasi.wasm .
rm *.wasi.wasm
