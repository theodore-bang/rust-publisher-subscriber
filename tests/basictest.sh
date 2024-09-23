#!/usr/bin/env bash

# timeout 5s ./target/debug/server > ./tests/basicserver.txt &
timeout 5s ./target/debug/server &

sleep 1

./target/debug/basicclient
