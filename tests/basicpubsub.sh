#!/usr/bin/env sh

timeout 7s ./target/debug/server > /dev/null &

sleep 1

./target/debug/basicpub &
./target/debug/basicsub & 

sleep 10
pkill -P $$