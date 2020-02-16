// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright (C) 2019 Doug P. Lau
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

use std::convert::TryInto;

use crate::{Fraction, Note, MeasureElem, GlyphId, Steps};

/// An iterator over durations of notes in a measure.  Should only output
/// correct notation.
pub(super) struct Notator<'a> {
    // Fraction of the measure that's been notated.
    pub(super) width: Fraction,
    //
    measure: &'a mut MeasureElem,
}

impl<'a> Notator<'a> {
    /// Create a new `Notator`
    pub(super) fn new(measure: &'a mut MeasureElem) -> Self {
        Notator {
            width: Fraction::new(0, 1),
            measure,
        }
    }

    /// Notate a note.
    pub(super) fn notate(&mut self, note: &Note) {
        // FIXME: Tuplets (test for not divisible by 128)
        let dur = ((note.duration.num as u32 * 128) / note.duration.den as u32)
            .try_into().unwrap();

        if note.pitch.is_empty() {
            self.notate_rest(dur);
        }
        let reset_width = self.width;
        for pitch_index in 0..note.pitch.len() {
            self.width = reset_width;
            self.notate_pitch(dur, note.visual_distance(pitch_index));
        }
    }

    // Notate a pitched note.
    fn notate_pitch(&mut self, mut dur: u16, visual_distance: Option<Steps>) {
        let mut check = 128;
        let temp_width = self.width + Fraction::new(dur, 128).simplify();
        self.width = temp_width;

        while dur != 0 {
            if dur == check {
                self.width = self.width - Fraction::new(check, 128).simplify();
                self.width = self.width.simplify();
                self.measure.add_pitch(check, self.width, visual_distance);
                dur -= check;
            } else if dur > check {
                self.width = self.width - Fraction::new(check, 128).simplify();
                self.width = self.width.simplify();
                self.measure.add_pitch(check, self.width, visual_distance);
                dur -= check;
            }

            check /= 2;
        }

        self.width = temp_width;
    }

    // Notate a rest.
    fn notate_rest(&mut self, mut dur: u16) {
        let mut check = 128;
        let temp_width = self.width + Fraction::new(dur, 128).simplify();
        self.width = temp_width;

        while dur != 0 {
            if dur == check {
                self.width = self.width - Fraction::new(check, 128).simplify();
                self.width = self.width.simplify();
                self.measure.add_rest(GlyphId::rest_duration(check), self.width);
                dur -= check;
            } else if dur > check {
                self.width = self.width - Fraction::new(check, 128).simplify();
                self.width = self.width.simplify();
                self.measure.add_rest(GlyphId::rest_duration(check), self.width);
                dur -= check;
            }

            check /= 2;
        }

        self.width = temp_width;
    }
}
