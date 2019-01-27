#!/bin/bash

set -ex

dune exec ocaml/workbench/main.exe "$1" | \
cargo run --manifest-path=implicit/Cargo.toml --bin server --release > "umbrellas/$1.svg"
