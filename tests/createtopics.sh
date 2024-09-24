#!/usr/bin/env sh

./target/debug/server > /dev/null &

while true; do
    sleep 1
    ./target/debug/createtopic
done &

sleep 6 
pkill -P $$
