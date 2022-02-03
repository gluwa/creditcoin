#!/bin/bash

while read -r line
do
  echo "$line" | iconv -c -t utf-8
done
