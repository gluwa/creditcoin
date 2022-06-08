#!/bin/bash

pushd ../creditcoin-js/ && yarn install && npm pack && popd
yarn install

./wait-for-creditcoin.sh $@
