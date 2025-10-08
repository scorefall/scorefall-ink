rustup toolchain install $1 --profile=minimal -c rustfmt && \
    echo "Checking format..." && \
    cargo +$1 fmt --all --check && \
    echo "Format check passed"
