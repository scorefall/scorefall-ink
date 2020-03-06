# Score File (*.scof)
Music score format developed for ScoreFall based on a zip file.

## Format
Format is in a zip file.  An example contents are in `scof/`.  A schema describing the format is in `schema/`.

## Language
Each file is based on [MuON (Micro Object Notation)](https://github.com/muon-data/muon/).  Details can be found in `schema`.

## About
Bars of music in scof must always have the correct number of beats.  Using functions provided in this crate you should not be able to get the wrong number of beats, or incorrect/hard-to-read notation.

### TODO
- Schema for synthesis (soundfont data, instruments)
- Schema for formatting (page breaks, line breaks, customized spacing, font choice)

## Scof Terminology
- Chan: Short for channel, an individual player / individual hand of a pianist.
- Stave: Stave or Staff (USA), can contain multiple Chans.  If Chans use
  different clefs, it becomes a grand stave.
- Part: Sheet music, can contains multiple Chans, like a Stave (Same thing
  except doesn't show in master score, just individual player's music).
- 

