#!/bin/bash

wasmfile="fibtest.wasm"

wasm-prune -e invoke $wasmfile  wasm_prune.wasm
/Users/sss/dev/rust_project/wabt/out/clang/Debug/wasm2wat $wasmfile  -o wasm_demo.wast
/Users/sss/dev/rust_project/wabt/out/clang/Debug/wasm2wat wasm_prune.wasm  -o wasm_prune.wast
/Users/sss/dev/rust_project/wabt/out/clang/Debug/wat2wasm wasm_prune.wast  -o wasm_prune_no_custom.wasm
/Users/sss/dev/rust_project/wabt/out/clang/Debug/wasm2wat wasm_prune_no_custom.wasm  -o wasm_prune_no_custom.wast
