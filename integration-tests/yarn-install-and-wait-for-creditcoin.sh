#!/bin/bash

pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn install

./wait-for-creditcoin.sh $@
