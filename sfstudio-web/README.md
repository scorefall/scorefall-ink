# ScoreFall™ Studio
User interface for ScoreFall™ Studio.

## Building from source
1. Install cargo-web, while you're waiting move on to step 2.
```bash
cargo install cargo-web --force
```
2. Make a new folder `scorefall/`
3. Run the following commands inside that folder:
```bash
git clone git@github.com:scorefall/scof.git
git clone git@github.com:scorefall/score2svg.git
git clone git@github.com:scorefall/scorefall-studio.git
git clone git@github.com:scorefall/scorefall.git
```
4. cd into scorefall-studio
5. `cargo web start` - And click on the link and it will open the app!

## Using the wasm32-unknown-unknown target
### Building
```
cargo web start --target=wasm32-unknown-unknown --release
```

### Releasing
```
cargo web deploy --target=wasm32-unknown-unknown --release
```
