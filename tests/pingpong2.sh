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

while true; do
    sleep 5
    ./target/debug/ping &
    ./target/debug/pong &
done &

sleep 60

kill $(list_descendants $$)
pkill -P $$

echo "All background processes have been terminated."
