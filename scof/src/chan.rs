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

use serde_derive::{Deserialize, Serialize};

use crate::Marking;

/// Channel information for a specific bar of music.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Chan {
    /// Channel notes for 1 bar.
    notes: String,
    /// Channel lyrics for 1 bar.
    lyric: Option<String>,
}

/// A parsed and transformed channel information for a specific bar of music.
#[derive(PartialEq, Debug)]
pub struct Channel {
    /// Channel notes for 1 bar.
    pub(crate) notes: Vec<Marking>,
    /// Channel lyrics for 1 bar.
    lyric: Option<String>,
}

impl Default for Chan {
    fn default() -> Self {
        let notes = String::new(); // no notes = whole measure rest
        let lyric = None;
        Chan { notes, lyric }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Chan::default().into()
    }
}

impl From<Chan> for Channel {
    fn from(chan: Chan) -> Self {
        let mut notes = vec![];

        for marking in chan.notes.split(' ').filter(|m| !m.is_empty()) {
            notes.push(marking.parse().unwrap_or_else(|_| {
                panic!("Invalid marking: {}", marking);
            }));
        }

        let lyric = chan.lyric;

        Channel { notes, lyric }
    }
}
