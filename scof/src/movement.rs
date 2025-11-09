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

use muon_rs as muon;
use serde_derive::{Deserialize, Serialize};

use crate::{Bar, Measure, Sig};

/// A movement in the score.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Mvmt {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Bar>,
}

impl Default for Mvmt {
    fn default() -> Mvmt {
        muon::from_str(include_str!("../res/default_movement.muon")).unwrap()
    }
}

/// A movement in the score.
#[derive(PartialEq, Debug)]
pub struct Movement {
    /// A list of key signatures used in this movement.
    pub sig: Vec<Sig>,
    /// Each measure of the movement in order.
    pub bar: Vec<Measure>,
}

impl Default for Movement {
    fn default() -> Movement {
        Mvmt::default().into()
    }
}

impl From<Mvmt> for Movement {
    fn from(mut mvmt: Mvmt) -> Movement {
        let sig = mvmt.sig;
        let mut bar = Vec::new();

        bar.extend(mvmt.bar.drain(..).map(|i| i.into()));

        Movement { sig, bar }
    }
}
