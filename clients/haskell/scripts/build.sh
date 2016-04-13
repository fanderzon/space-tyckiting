#!/bin/sh

set -exu

docker run --rm=true -v $(pwd):/build phadej/stackage:1.11 sh /build/scripts/buildindocker.sh
docker build -t client/timid --file scripts/Dockerfile.timid .
docker build -t client/joker --file scripts/Dockerfile.joker .