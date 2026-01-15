#!/bin/sh
# This script builds the zj-handful zellij plugin and loads it in zellij for
# development. This script is meant to be run from the zestty project root.
set -e

if [ -n "$1" ] && [ "$1" = "release" ]; then
    cargo build --release --features tracing
    zellij plugin --skip-plugin-cache -- file:target/wasm32-wasip1/release/zj-handful.wasm
else
    cargo build --features tracing
    zellij plugin --skip-plugin-cache -- file:target/wasm32-wasip1/debug/zj-handful.wasm
fi

