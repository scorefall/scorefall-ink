# ScoreFall
Music composition program.

## Roadmap to first stable iteration (0.1.0)
- [ ] [scorefall-ui](https://github.com/scorefall/scorefall-ui) the actual program (putting everything together).
  - [ ] Start building WASM application in Rust for [scorefall-ui](https://github.com/scorefall/scorefall-ui).
  - [ ] [musicxml](https://github.com/scorefall/musicxml) parser with [quick-xml](https://crates.io/crates/quick-xml).
    - [ ] Translation to Scof
      - [ ] Score metadata iterator
      - [ ] Instrument & Notes iterators
  - [ ] [scof](https://github.com/scorefall/scof) structs of musical notation.
    - [x] Add score metadata struct
    - [x] Add instrument & note structs
    - [ ] Scof Parser with Muon
      - [x] Muon Specification
      - [ ] Muon Parser
  - [ ] [score2svg](https://github.com/scorefall/score2svg) music scores in [scof](https://github.com/scorefall/scof) structs into SVG files.
    - [x] Convert Bravura font into SVG defs.
    - [x] Rendering.
    - [x] Depends on `scof` for music structs.
    - [ ] API to take `scof` music structs and render it.
    - [ ] Be able to render all supported glyphs.
  - [ ] [scorefall](https://github.com/scorefall/scorefall)
    - [ ] Functions for editing score.
