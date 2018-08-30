#/bin/sh

cd ocaml
dune exec testgen/main.exe
cd ../testsuite
cargo expect run

