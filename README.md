# ScoreFall
Music composition program.

## Roadmap to first stable iteration (0.1.0)
- [ ] [scorefall-ui](https://github.com/scorefall/scorefall-ui) the actual program (putting everything together).
  - [ ] [musicxml](https://github.com/scorefall/musicxml) parser with [quick-xml](https://crates.io/crates/quick-xml).
    - [ ] Score metadata iterator
    - [ ] Instrument & Notes iterators
  - [ ] [scorefall](https://github.com/scorefall/scorefall) structs of musical notation.
    - [ ] Add score metadata struct
    - [ ] Add instrument & note structs
  - [ ] [score2svg](https://github.com/scorefall/score2svg) music scores in [scorefall](https://github.com/scorefall/scorefall) structs into SVG files.
    - [ ] Convert Emmentaler font into SVG markers
    - [ ] Depends on `scorefall`
    - [ ] Rendering
