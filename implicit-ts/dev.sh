#!/bin/bash

mkdir -p out
./node_modules/.bin/tsc ./test.ts --outDir ./out
node ./out/test.js
