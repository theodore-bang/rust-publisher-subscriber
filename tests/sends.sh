
#!/usr/bin/env sh

./target/debug/server &

./target/debug/send &

sleep 10

pkill -P $$