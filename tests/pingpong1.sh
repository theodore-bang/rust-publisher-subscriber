#!/usr/bin/env sh

list_descendants ()
{
  local children=$(ps -o pid= --ppid "$1")

  for pid in $children
  do
    list_descendants "$pid"
  done

  echo "$children"
}

./target/debug/server > /dev/null &

sleep 1

./target/debug/ping & 
./target/debug/pong &

sleep 5

kill $(list_descendants $$)

echo "All background processes have been terminated."
