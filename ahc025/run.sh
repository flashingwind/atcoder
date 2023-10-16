#!/usr/bin/env bash
rm -f out.txt
cargo run -r --bin tester cargo run --bin ahc025-a < $1 >out.txt
