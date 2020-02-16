# ScoreFall Studio
Music composition program.

## Crates In This Repository
- [sfstudio-web](https://github.com/scorefall/scorefall-studio/tree/master/sfstudio-web)
  the actual program (putting everything together).
- [scof](https://github.com/scorefall/scorefall-studio/tree/master/scof)
  structs of musical notation.
- [muflor](https://github.com/scorefall/scorefall-studio/tree/master/muflor) music scores in [scof](https://github.com/scorefall/scorefall-studio/tree/master/scof) structs into SVG files.
- [scorefall-studio](https://github.com/scorefall/scorefall-studio/tree/master/scorefall-studio)

## Developing
cargo-web can automatically build and run the web GUI as a server on
your machine for testing.

### Install
```bash
cargo install cargo-web
```

### Start Webserver
```bash
cd sfstudio-web
cargo web start --target=wasm32-unknown-unknown --release
```
