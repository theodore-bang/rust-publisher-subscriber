#!/usr/bin/env sh

timeout 6s ./target/debug/server > ./tests/pingpong1server.txt &

sleep 1

timeout 5s ./target/debug/ping & 
timeout 5s ./target/debug/pong &
