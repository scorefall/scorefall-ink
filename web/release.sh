RUSTFLAGS="--remap-path-prefix=$PWD=_ --remap-path-prefix=$HOME/.local/lib/cargo=- --remap-path-prefix=$HOME/.local/lib/rustup=+ --remap-path-prefix=$HOME=%" cargo build --target wasm32-unknown-unknown --release && \
    wasm-bindgen --out-dir site/gen --target web --no-typescript --remove-name-section --remove-producers-section --omit-default-module-path ../target/wasm32-unknown-unknown/release/ink.wasm && \
    wasm-opt site/gen/ink_bg.wasm -o site/gen/ink_bg.wasm -Os && \
    wasm-strip site/gen/ink_bg.wasm && \
    ls -l site/gen/ink_bg.wasm && \
    http site
