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

/* ************************************************************************** */

//! Render beams for beamed groups.

use scof::{Pitch, Steps};

// Beaming rules for a time signature
struct BeamRules {
    // 8ths
    eighth: u16,
    // 16ths
    sixteenth: u16,
    // 32nds (inner groupings of 4, outer eighth beam only)
    inner: u16,
}

// 4/4 Time signature beaming rules.
const BEAMRULE_4_4: BeamRules = BeamRules {
    eighth: 64,
    sixteenth: 32,
    inner: 16,
};

/// Should there be a beam connecting to previous note?
pub enum BeamProp {
    None,
    ContinueEighth,
    ContinueSixteenth,
    ContinueInner,
    Flag,
}

/// All of the beams in a measure.
pub(crate) struct Beams {
    // Duration not notated yet in the measure.
    dur: u16,
    // Notes that may be flagged or beamed.
    short: Vec<(BeamProp, u16, f32, (Vec<Pitch>, Steps))>,
}

impl Beams {
    /// Create an empty instance of beams for the measure.
    pub fn new() -> Self {
        Beams {
            // Start with 4 beats left (4/4)
            dur: 128,
            // Start with no discovered flag/beam notes yet.
            short: vec![],
        }
    }

    /// Advance duration.
    pub fn advance(&mut self, dur: u16, width: f32, y: Option<(Vec<Pitch>, Steps)>) {
        // Not a rest
        if let Some(y) = y {
            // Less than a quarter note
            if dur < 32 {
                let prop = BeamProp::Flag;

                self.short.push((prop, dur, width, y));
            } else {
            }
        }
        // Reduce remaining duration.
        self.dur -= dur;
    }
}

impl Iterator for Beams {
    type Item = Short;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((prop, dur, width, y)) = self.short.pop() {
            match prop {
                BeamProp::None => todo!(),
                BeamProp::ContinueEighth => todo!(),
                BeamProp::ContinueSixteenth => todo!(),
                BeamProp::ContinueInner => todo!(),
                BeamProp::Flag => {
                    return Some(Short::Flag(dur, width, y));
                },
            }
        }
        None
    }
}

/// Short note: A flag or a beam
pub(crate) enum Short {
    /// Flag
    Flag(u16, f32, (Vec<Pitch>, Steps)),
    /// Beam
    Beam(Beam),
}

/// A single beam
pub(crate) struct Beam {
    
}
