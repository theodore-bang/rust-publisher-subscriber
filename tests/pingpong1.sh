#!/usr/bin/env sh

timeout 10s ./target/debug/server > ./tests/pingpong1server.txt &

sleep 1

timeout 9s ./target/debug/ping & 
timeout 9s ./target/debug/pong &
