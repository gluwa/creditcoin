#!/bin/bash

pushd ../creditcoin-js/ && yarn install && popd
yarn install

./wait-for-creditcoin.sh $@
