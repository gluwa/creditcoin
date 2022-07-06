#!/bin/bash

pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn --update-checksums
yarn install

./wait-for-creditcoin.sh $@
