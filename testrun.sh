# !/usr/bin/sh

cd server
node start-server.js&
sleep 1
cd ../clients/serenity
cargo build
cargo run&
cargo run&
