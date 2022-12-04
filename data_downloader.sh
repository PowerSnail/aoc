#!/bin/bash

set -o allexport
source .env
set +o allexport

year=$1
day=$2

curl \
    -o "inputs/${year}/day${day}.txt" \
    --cookie "${AOC_COOKIE}" \
    "https://adventofcode.com/${year}/day/${day}/input"
