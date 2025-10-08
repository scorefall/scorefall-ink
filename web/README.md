# ScoreFall™ Ink Web

Web UI for ScoreFall™ Ink.

## Developing

### Prerequisites

Install [wabt](https://github.com/WebAssembly/wabt); If you're on Fedora, you
can do this by running:

```shell
sudo dnf install wabt
```

Install the wasm-bindgen CLI and wasm-opt with `cargo`:

```shell
cargo install wasm-bindgen-cli --locked
cargo install wasm-opt --locked
```

### Iterative Development

Build the debug binary and start webserver with:

```shell
./debug.sh
```

### Testing Release Build

Build the release binary and start webserver with:

```shell
./release.sh
```
