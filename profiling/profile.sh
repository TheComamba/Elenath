#!/bin/bash

if ! command -v perf &> /dev/null
then
    echo "Installing perf"
    sudo aptitude install linux-tools-common linux-tools-generic
fi

if ! command -v cargo-flamegraph &> /dev/null
then
    echo "Installing cargo-flamegraph"
    cargo install flamegraph
fi

#go to git root directory
cd $(git rev-parse --show-toplevel)

export CARGO_PROFILE_RELEASE_DEBUG=true
echo 0 | sudo tee "/proc/sys/kernel/perf_event_paranoid"

cargo flamegraph -- -g

rm perf.data
mv flamegraph.svg profiling