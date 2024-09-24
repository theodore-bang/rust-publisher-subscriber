#!/usr/bin/env sh

timeout 20s ./target/debug/server > ./tests/pingpong1server.txt &

sleep 1

timeout 15s ./target/debug/ping & 
timeout 15s ./target/debug/pong &
