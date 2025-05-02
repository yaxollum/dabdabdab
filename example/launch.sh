#!/bin/bash
cargo build --release
# spin up 4 workers
for i in $(seq 1 4);
do
    echo Launched worker $i
    target/release/dabdabdab myworker &> /dev/null &
done
