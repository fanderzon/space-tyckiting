#!/bin/sh

set -exu

cd /build
rm -rf .cabal-sandbox cabal.sandbox.config
cabal clean
cabal install --only-dependencies
cabal build
chmod -R a+rw dist
