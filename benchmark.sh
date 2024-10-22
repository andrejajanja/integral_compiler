#!/bin/bash
perf record -e cycles,cache-misses,L1-dcache-load-misses -F 10000 --call-graph dwarf ./target/release/prototype