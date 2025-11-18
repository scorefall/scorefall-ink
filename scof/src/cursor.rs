// ScoreFall Ink - Music Composition Software
//
// Copyright (C) 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
// Copyright (C) 2019-2025 Doug P. Lau
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
//
use crate::Scof;

/// Cursor pointing to a marking
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Cursor {
    /// Movement number at cursor
    pub(crate) movement: u16,
    /// Bar number at cursor
    pub(crate) bar: u16,
    /// Channel number at cursor
    pub(crate) chan: u16,
    /// Marking number within bar
    pub(crate) marking: u16,
}

impl Cursor {
    /// Create a new cursor
    #[must_use]
    pub fn new(movement: u16, bar: u16, chan: u16, marking: u16) -> Self {
        Cursor {
            movement,
            bar,
            chan,
            marking,
        }
    }

    /// Create a cursor from a chan #.
    #[must_use]
    pub fn chan(self, chan: u16) -> Self {
        Cursor {
            movement: self.movement,
            bar: self.bar,
            chan,
            marking: self.marking,
        }
    }

    /// Create a cursor one marking to the left, shifting bar if necessary.
    #[must_use]
    pub fn left(mut self, scof: &Scof) -> Self {
        if self.marking > 0 {
            self.marking -= 1;
        } else if self.bar > 0 {
            self.bar -= 1;
            let len = scof.marking_len(self);
            self.marking = if len > 0 { len - 1 } else { 0 };
        }
        self
    }

    /// Create a cursor one marking to the right, shifting bar if necessary.
    #[must_use]
    pub fn right(mut self, scof: &Scof) -> Self {
        self.marking += 1;
        if self.marking >= scof.marking_len(self) {
            self.bar += 1;
            self.marking = 0;
        }
        self
    }

    /// Create a cursor one marking to the right, not checking if bar ended.
    #[must_use]
    pub fn right_unchecked(mut self) -> Self {
        self.marking += 1;
        self
    }

    /// Returns true if it's the first bar of music.
    #[must_use]
    pub fn is_first_bar(self) -> bool {
        self.bar == 0
    }
}
