#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

function compile {
    SCRIPT=$1
    echo "compiling $SCRIPT"
    SCRIPT_DIR=$(dirname $1)
    OUT_TEMP=$SCRIPT_DIR/out
    OUT_JSON=$SCRIPT_DIR/$(basename $SCRIPT .ts).json

    $DIR/../implicit-ts/node_modules/.bin/tsc -t ES5 $SCRIPT --outDir $OUT_TEMP
    SCRIPT=$(find $OUT_TEMP | grep -v implicit-ts | grep ".js$")
    node $SCRIPT > $OUT_JSON
    rm -rf $OUT_TEMP
}

for script in $(find ./tests/ -path ".*.ts")
do
    # compiles the script in a background process
    compile $script &
done

wait # waits for background jobs to finish
