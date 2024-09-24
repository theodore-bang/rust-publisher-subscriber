#!/usr/bin/env sh

timeout 20s ./target/debug/server > ./tests/pingpong2server.txt &

sleep 1

timeout 15s ./target/debug/ping & 
timeout 15s ./target/debug/pong &

sleep 1

timeout 15s ./target/debug/ping & 
timeout 15s ./target/debug/pong &

sleep 1

timeout 15s ./target/debug/ping & 
timeout 15s ./target/debug/pong &

sleep 1

timeout 15s ./target/debug/ping & 
timeout 15s ./target/debug/pong &

sleep 20s
echo "Done!"
pkill -P $$
