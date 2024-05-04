#!/usr/bin/env sh
./scripts/build.sh "$1" && mkdir -p "./output/" && cp "./target/$1/release/dstub" "./output/dstub" && cp LICENSE "./output/" && mkdir -p "./output/"