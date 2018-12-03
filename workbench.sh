#!/bin/bash

set -ex

dune exec ocaml/workbench/main.exe | \
cargo run --manifest-path=implicit/Cargo.toml --bin server --release \
> out.svg
