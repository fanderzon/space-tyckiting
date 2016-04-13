#!/bin/sh

set -e
set -x

# Prepare sandbox with Stackage (should compile fine)
mkdir install-sandbox
cd install-sandbox

# Let's start with LTS Stackage
curl -O http://www.stackage.org/lts/cabal.config

# 0.9.13 has ellipsis support!
sed -ie 's/doctest ==0.9.12/doctest ==0.9.13/' cabal.config

# we don't need those
echo 'library-profiling: False' >> cabal.config
echo 'documentation: False' >> cabal.config

# We'll work in sandbox
cabal sandbox init

# Grab a coffee for the install
cabal install --datadir=/usr/local/share \
  cabal-db \
  cabal-install \
  djinn \
  doctest \
  ghc-mod \
  hasktags \
  hdevtools \
  hlint \
  structured-haskell-mode \
  stylish-haskell \
  && echo "Installed!"

# Copy binaries into path
sudo cp .cabal-sandbox/bin/* /usr/local/bin

# Cleanup
cd ..
rm -rf install-sandbox
