#!/usr/bin/env sh

./target/debug/server &

sleep 1

./target/debug/basicpub &
./target/debug/basicsub & 

sleep 5
pkill -P $$