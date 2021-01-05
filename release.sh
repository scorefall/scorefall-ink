#!/usr/bin/sh

wasm-pack build --no-typescript --target web --out-name site --release web --out-dir site/gen && firebase deploy
