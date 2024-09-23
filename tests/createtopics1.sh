#!/usr/bin/env sh

timeout 5s ./target/debug/server &

sleep 1

./target/debug/topiccreator
