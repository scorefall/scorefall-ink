// ScoreFall Ink - Music Composition Software
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

use sfff::{Glyph, SfFontMetadata};

/// Different styles of noteheads
pub enum Notehead {
    Normal,
    X,
    Diamond,
    Triangle,
    Slash,
}

/// Get width of the notehead.
pub(super) fn width(
    notehead: Notehead,
    meta: &SfFontMetadata,
    duration: u16,
) -> i32 {
    let [[left_x, _], [right_x, _]] = stems(notehead, meta, duration);
    right_x - left_x
}

/// Get left and right stem positions
pub(super) fn stems(
    notehead: Notehead,
    meta: &SfFontMetadata,
    duration: u16,
) -> [[i32; 2]; 2] {
    use Notehead::*;

    let double;
    let whole;
    let half;
    let fill;
    match notehead {
        Normal => {
            double = meta.notehead_double;
            whole = meta.notehead_whole;
            half = meta.notehead_half;
            fill = meta.notehead;
        }
        X => {
            double = meta.notehead_double_x;
            whole = meta.notehead_whole_x;
            half = meta.notehead_half_x;
            fill = meta.notehead_x;
        }
        Diamond => {
            double = meta.notehead_double_diamond;
            whole = meta.notehead_whole_diamond;
            half = meta.notehead_half_diamond;
            fill = meta.notehead_diamond;
        }
        Triangle => {
            double = meta.notehead_double_triangle;
            whole = meta.notehead_whole_triangle;
            half = meta.notehead_half_triangle;
            fill = meta.notehead_triangle;
        }
        Slash => {
            double = meta.notehead_double_slash;
            whole = meta.notehead_whole_slash;
            half = meta.notehead_half_slash;
            fill = meta.notehead_slash;
        }
    }

    variants(double, whole, half, fill, duration)
}

/// Get the notehead glyph for a note with a specific duration
pub(super) fn duration(duration: u16) -> Glyph {
    use Glyph::*;
    variants(
        NoteheadDouble,
        NoteheadWhole,
        NoteheadHalf,
        NoteheadFill,
        duration,
    )
}

/// Get the notehead glyph for a note with a specific duration
pub(super) fn x_duration(duration: u16) -> Glyph {
    use Glyph::*;
    variants(
        NoteheadDoubleX,
        NoteheadWholeX,
        NoteheadHalfX,
        NoteheadFillX,
        duration,
    )
}

/// Get the square notehead glyph for a note with a specific duration
pub(super) fn slashed_duration(duration: u16) -> Glyph {
    use Glyph::*;
    variants(
        NoteheadDoubleSlashed,
        NoteheadWholeSlashed,
        NoteheadHalfSlashed,
        NoteheadFillSlashed,
        duration,
    )
}

/// Get the large square notehead glyph for a note with a specific duration
pub(super) fn slash_duration(duration: u16) -> Glyph {
    use Glyph::*;
    variants(
        NoteheadDoubleSlash,
        NoteheadWholeSlash,
        NoteheadHalfSlash,
        NoteheadFillSlash,
        duration,
    )
}

/// Given a duration and set of notehead glyphs, choose appropriate glyph
fn variants<T>(double: T, whole: T, half: T, fill: T, duration: u16) -> T {
    match duration {
        1..=63 => fill,
        64..=127 => half,
        128..=255 => whole,
        _ => double,
    }
}
