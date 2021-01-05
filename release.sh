#!/usr/bin/sh

wasm-pack build --no-typescript --target web --out-dir web/site/gen --out-name site --release web && firebase deploy
