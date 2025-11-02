rustup toolchain install $1 --profile=minimal && \
    echo "Running tests..." && \
    cargo +$1 test --all --all-features && \
    echo "Tests passed"
