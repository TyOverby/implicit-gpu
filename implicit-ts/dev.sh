#!/bin/bash

mkdir -p out
./node_modules/.bin/tsc -t ES5 ./test.ts --outDir ./out
node ./out/test.js
