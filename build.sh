#!/bin/bash

export rustc_version=$(rustc --version)
if [[ "$1" == '--release' ]]; then
    cargo clippy && cargo build --release
else
    cargo clippy && cargo build 
fi