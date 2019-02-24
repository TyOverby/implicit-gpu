#!/bin/bash

set -ex

dune exec frontend/ocaml/workbench/main.exe "$1" | \
cargo run --manifest-path=core/implicit/Cargo.toml --bin server --release > "umbrellas/$1.svg"
