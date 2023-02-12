#!/usr/bin/env bash

set -eu

cargo +nightly contract build --manifest-path othercontract/Cargo.toml
cargo +nightly contract build
