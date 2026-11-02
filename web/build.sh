#!/usr/bin/env bash
# Builds the playground into web/dist: the static files from web/www plus the WASM glue in dist/pkg.
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli at the version pinned in Cargo.toml.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --locked --target wasm32-unknown-unknown
rm -rf dist
mkdir dist
cp -R www/. dist/
wasm-bindgen --target web --no-typescript --out-dir dist/pkg \
    target/wasm32-unknown-unknown/release/gsichtl_web.wasm
