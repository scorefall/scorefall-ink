// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019-2020 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
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

use scof::{Cursor, Marking, Pitch, Scof};

/// An iterator over durations of notes in a measure.  Should only output
/// correct notation.  (Turns 3/8 into dotted 1/4 or 1/4 tied to 1/8 depending
/// on what's appropriate).
pub(super) struct Notator<'a> {
    // Cursor through the notes.
    curs: Cursor,
    // Duration left of current note (may be note tied to another)
    dur: u16,
    // Note to check duration against
    check: u16,
    //
    scof: &'a Scof,
    //
    pitch: Vec<Pitch>,
    // User's cursor
    cursor: Cursor,
    // Is User's Cursor
    ic: bool,
}

impl<'a> Notator<'a> {
    /// Create a new `Notator`
    pub(super) fn new(scof: &'a Scof, cursor: Cursor, curs: Cursor) -> Self {
        Notator {
            curs,
            dur: 0,
            check: 128,
            scof,
            pitch: vec![],
            cursor,
            ic: false,
        }
    }

    pub(super) fn is_cursor(&self) -> bool {
        self.curs == self.cursor
    }
}

impl<'a> Iterator for Notator<'a> {
    type Item = (Vec<Pitch>, u16, bool);

    fn next(&mut self) -> Option<Self::Item> {
        // If duration is not 0, find next note to add.
        while self.dur != 0 {
            if self.dur >= self.check {
                self.dur -= self.check;
                return Some((self.pitch.clone(), self.check, self.ic));
            }
            self.check /= 2;
        }
        // Get next note/rest, return None if done.
        match self.scof.marking(&self.curs)? {
            Marking::Note(note) => {
                self.ic = self.curs == self.cursor;
                self.check = 128;
                // FIXME: Tuplets (test for not divisible by 128)
                self.dur = ((note.duration.num as u32 * 128)
                    / note.duration.den as u32)
                    .try_into()
                    .unwrap();
                self.pitch = note.pitch.clone();
            }
            _ => unreachable!(),
        };
        self.curs.right_unchecked();
        <Self as Iterator>::next(self)
    }
}
