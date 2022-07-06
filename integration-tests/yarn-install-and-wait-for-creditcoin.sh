#!/bin/bash

pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn --update-checksums
yarn upgrade

./wait-for-creditcoin.sh $@
