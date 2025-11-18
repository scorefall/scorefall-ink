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

use crate::{Chan, Channel};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SigRef {
    /// Index into sig list.
    index: u32,
    /// Which beat of the measure the signature starts applying to.
    beat: Option<u8>,
}

/// A bar (or measure) of music.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    /// Signature reference (index)
    sig: Option<SigRef>,
    /// All of the channels in this piece.
    chan: Vec<Chan>,
    /// Repeat symbols for this measure.
    repeat: Vec<String>,
}

/// A bar (or measure) of music.
#[derive(Debug, PartialEq)]
pub struct Measure {
    /// Signature reference (index)
    pub sig: Option<SigRef>,
    /// All of the channels in this piece.
    pub chan: Vec<Channel>,
    /// Repeat symbols for this measure.
    pub repeat: Vec<String>,
}

impl From<Bar> for Measure {
    fn from(mut bar: Bar) -> Self {
        let mut chan = vec![];

        for i in bar.chan.drain(..) {
            chan.push(i.into());
        }

        let sig = bar.sig;
        let repeat = bar.repeat;

        Measure { sig, chan, repeat }
    }
}
