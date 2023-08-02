#!/bin/bash

export rustc_version=$(rustc --version)
if [[ "$1" == '--release' ]]; then
    cargo build --release
else
    cargo build 
fi