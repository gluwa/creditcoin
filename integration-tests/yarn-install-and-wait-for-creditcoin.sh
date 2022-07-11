#!/bin/bash

pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn upgrade 'creditcoin-js'

./wait-for-creditcoin.sh $@
