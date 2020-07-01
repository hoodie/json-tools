#!/usr/bin/bash

cargo wasi build --release
cp target/wasm32-wasi/release/*.wasi.wasm wasm
rm wasm/*.wasi.wasm
