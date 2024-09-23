#!/usr/bin/env sh

command="echo -e "hi" & echo -e "he" & echo -e "wo""

timeout 1s $command --fore-ground

