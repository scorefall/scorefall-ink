rustup toolchain install $1 --profile=minimal -c rustfmt
cargo +$1 fmt --all --check
