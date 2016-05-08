# !/usr/bin/sh

cd server
node start-server.js > testrun_server_log.txt &
sleep 1
cd ../clients/serenity
cargo run > testrun_client1_log.txt &
sleep 1
cargo run > testrun_client2_log.txt &
