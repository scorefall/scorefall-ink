# ScoreFall™ Ink
Music composition, arrangement, engraving, and notation program.

## Crates In This Repository
- [scorefall-ink-web](https://github.com/scorefall/scorefall-ink/tree/master/sfink-web)
  the actual program (putting everything together).
- [scof](https://github.com/scorefall/scorefall-ink/tree/master/scof)
  structs of musical notation.
- [staverator](https://github.com/scorefall/scorefall-ink/tree/master/staverator) music scores in [scof](https://github.com/scorefall/scorefall-ink/tree/master/scof) structs into SVG files.
- [scorefall-ink](https://github.com/scorefall/scorefall-ink/tree/master/scorefall-ink)

## Developing
cargo-web can automatically build and run the web GUI as a server on
your machine for testing.

### Install
```bash
cargo install cargo-web
```

### Start Webserver
```bash
cd scorefall-ink-web
cargo web start --target=wasm32-unknown-unknown --release
```
