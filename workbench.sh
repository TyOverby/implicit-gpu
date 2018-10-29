#!/bin/bash

set -ex

dune exec ocaml/workbench/main.exe | \
cargo run --manifest-path=implicit/Cargo.toml --release \
> out.svg
