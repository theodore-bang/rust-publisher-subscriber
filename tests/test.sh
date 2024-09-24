#!/usr/bin/env sh

./target/debug/server &

while true; do 
	timeout 5s sleep 5 &
done &

sleep 10

pkill -P $$
