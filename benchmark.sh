#!/bin/bash
perf record -o benchmarks/perf.data -e cycles  -F 5000 --call-graph dwarf ./target/release/prototype

#sudo sysctl -w kernel.kptr_restrict=0
#sudo sysctl -w kernel.perf_event_paranoid=0