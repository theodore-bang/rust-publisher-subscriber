#!/usr/bin/env sh

timeout 5s ./target/debug/server &
./target/debug/basicpub
./target/debug/basicsub
