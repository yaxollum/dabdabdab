#!/bin/bash
cargo build --release
# spin up 4 workers
for i in $(seq 1 4);
do
    target/release/dabdabdab myworker &
done
