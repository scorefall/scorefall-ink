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

use scof::Scof;

/// This is the entire program context.
pub struct Program {
    /// The save file.
    pub scof: Scof,
    // Which channel is the cursor at.
    pub chan: usize,
    // Which bar is the cursor at.
    pub bar: usize,
    // Cursor within the measure (not necessarily at a specific time).
    pub curs: usize,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            scof: Scof::default(),
            bar: 0,
            curs: 0,
            chan: 0,
        }
    }
}

impl Program {
    /// Create a new program.
    pub fn new() -> Self {
        Self::default()
    }

    /// Move cursor back.
    pub fn left(&mut self) {
        if self.curs > 0 {
            self.curs -= 1;
        } else if self.bar != 0 {
            self.bar -= 1;
            self.curs = 0;
            while self
                .scof
                .marking(self.bar, self.chan, self.curs + 1)
                .is_some()
            {
                self.curs += 1;
            }
        }
    }

    /// Move cursor forward.
    pub fn right(&mut self) {
        if self
            .scof
            .marking(self.bar, self.chan, self.curs + 1)
            .is_some()
        {
            self.curs += 1;
        } else {
            // Measure has ended.
            self.bar += 1;
            self.curs = 0;
            if self.scof.marking(self.bar, self.chan, self.curs).is_none() {
                // Measure doesn't exist, so make a new one.
                self.scof.new_measure();
            }
        }
    }
}
