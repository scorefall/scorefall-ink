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
    chan: usize,
    // Which bar is the cursor at.
    bar: usize,
    // Cursor within the measure (not necessarily at a specific time).
    curs: usize,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
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
