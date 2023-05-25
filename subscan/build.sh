#!/bin/bash
set -euo pipefail

function build() {
    go build -o ./cmd/subscan -v ./cmd
	echo "Build Success"
}

function help() {
    echo "usage: ./build.sh build"
}

if [[ "$1" == "" ]]; then
    help
elif [[ "$1" == "build" ]];then
    build
fi
