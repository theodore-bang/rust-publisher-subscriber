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
    sleep 2
    ./target/debug/pull &
done &

sleep 90

kill $(list_descendants $$)

echo "All background processes have been terminated."