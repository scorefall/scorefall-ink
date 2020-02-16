# Score File (*.scof)
Music score format developed for ScoreFall based on a zip file.

## Format
Format is in a zip file.  An example contents are in `scof/`.  A schema describing the format is in `schema/`.

## Language
Each file is based on [MuON (Micro Object Notation)](https://github.com/DougLau/muon/).  Details can be found in `schema`.

## About
Bars of music in scof must always have the correct number of beats.  Using functions provided in this crate you should not be able to get the wrong number of beats, or incorrect/hard-to-read notation.

### TODO
- Schema for synthesis (soundfont data, instruments)
- Schema for formatting (page breaks, line breaks, customized spacing, font choice)
