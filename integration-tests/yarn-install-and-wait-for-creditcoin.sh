#!/bin/bash

pushd ../creditcoin-js/ && yarn install && yarn pack && popd || exit 1
yarn upgrade 'creditcoin-js'

./wait-for-creditcoin.sh "$@"
