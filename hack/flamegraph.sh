#!/bin/bash
set -ex
echo "detecting running void instance"
PID=$(pgrep void)
perf record -F 99 -p "$PID" -g -- sleep 60 || true
perf script > out.perf
./stackcollapse-perf.pl out.perf > out.folded
./flamegraph.pl out.folded > flamegraph.svg
rm perf.data out.perf out.folded
