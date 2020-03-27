// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019-2020 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright (C) 2019-2020 Doug P. Lau
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(unused)] // FIXME: For now, until all of the glyphs are implemented.

use scof::Fraction;
use sfff::Glyph;

/// Get the glyph for a rest with a specific duration
pub(super) fn rest_duration(duration: u16) -> Glyph {
    use Glyph::*;
    match duration {
        2 | 3 => Rest64,
        4 | 6 | 9 => Rest32,
        8 | 12 | 18 | 27 => Rest16,
        16 | 24 | 36 | 54 | 81 => Rest8,
        32 | 48 | 72 | 108 | 162 => Rest4,
        64 | 96 | 144 | 216 => Rest2,
        128 | 192 | 288 => Rest1,
        256 | 384 => Rest1, // FIXME: Double Whole Rest
        512 => Rest1,       // FIXME: Quadruple Whole Rest
        _ => panic!("Bug in Notator, no glyph for ({})", duration),
    }
}

/// Get the flag glyph for a note with a specific duration
pub(super) fn flag_duration(duration: u16, up: bool) -> Option<Glyph> {
    use Glyph::*;

    Some(match duration {
        2 | 3 => {
            if up {
                FlagUp64
            } else {
                FlagDown64
            }
        }
        4 | 6 | 9 => {
            if up {
                FlagUp32
            } else {
                FlagDown32
            }
        }
        8 | 12 | 18 | 27 => {
            if up {
                FlagUp16
            } else {
                FlagDown16
            }
        }
        16 | 24 | 36 | 54 | 81 => {
            if up {
                FlagUp8
            } else {
                FlagDown8
            }
        }
        // All other longer durations don't have flags.
        _ => return None,
    })
}

/// Get the notehead glyph for a note with a specific duration
pub(super) fn notehead_duration(duration: u16) -> Glyph {
    use Glyph::*;
    notehead_variants(
        NoteheadDouble,
        NoteheadWhole,
        NoteheadHalf,
        NoteheadFill,
        duration,
    )
}

/// Get the notehead glyph for a note with a specific duration
pub(super) fn x_notehead_duration(duration: u16) -> Glyph {
    use Glyph::*;
    notehead_variants(
        NoteheadDoubleX,
        NoteheadWholeX,
        NoteheadHalfX,
        NoteheadFillX,
        duration,
    )
}

/// Get the square notehead glyph for a note with a specific duration
pub(super) fn slashed_notehead_duration(duration: u16) -> Glyph {
    use Glyph::*;
    notehead_variants(
        NoteheadDoubleSlashed,
        NoteheadWholeSlashed,
        NoteheadHalfSlashed,
        NoteheadFillSlashed,
        duration,
    )
}

/// Get the large square notehead glyph for a note with a specific duration
pub(super) fn slash_notehead_duration(duration: u16) -> Glyph {
    use Glyph::*;
    notehead_variants(
        NoteheadDoubleSlash,
        NoteheadWholeSlash,
        NoteheadHalfSlash,
        NoteheadFillSlash,
        duration,
    )
}

/// Given a duration and set of notehead glyphs, choose appropriate glyph
fn notehead_variants(
    double: Glyph,
    whole: Glyph,
    half: Glyph,
    fill: Glyph,
    duration: u16,
) -> Glyph {
    match duration {
        1..=63 => fill,
        64..=127 => half,
        128..=255 => whole,
        _ => double,
    }
}
