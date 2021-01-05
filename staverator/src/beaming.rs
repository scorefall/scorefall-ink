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

/* ************************************************************************** */

//! Render beams for beamed groups.

use std::collections::VecDeque;

use scof::{Pitch, Steps};

use cala::log::{Tag, log};

const INFO: Tag = Tag::new("Beaming");

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
#[derive(PartialEq, Debug)]
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
    short: VecDeque<(BeamProp, u16, f32, (Vec<Pitch>, Steps))>,
    // Last was short?
    last_short: bool,
    // Minimum duration within current beam.
    min_dur: u16,
    // Notes in the beamed group.
    notes: Vec<(u16, f32, (Vec<Pitch>, Steps), bool)>,
    // For iterator.
    queued: Option<Short>,
}

impl Beams {
    /// Create an empty instance of beams for the measure.
    pub fn new() -> Self {
        Beams {
            // Start with 4 beats left (4/4)
            dur: 128,
            // Start with no discovered flag/beam notes yet.
            short: VecDeque::new(),
            //
            last_short: false,
            //
            min_dur: 0,
            //
            notes: vec![],
            //
            queued: None,
        }
    }

    /// Advance duration.
    pub fn advance(
        &mut self,
        dur: u16,
        width: f32,
        y: Option<(Vec<Pitch>, Steps)>,
    ) {
        let new_dur = self.dur - dur;
        // Not a rest
        self.last_short = if let Some(y) = y {
            // Less than a quarter note
            if dur < 32 {
                let prop = if self.last_short
                    && self.dur / BEAMRULE_4_4.eighth
                        == new_dur / BEAMRULE_4_4.eighth
                {
                    // If last note could be beamed to this note
                    let mut prev = self.short.pop_back().unwrap();
                    if prev.0 == BeamProp::Flag {
                        prev.0 = BeamProp::None;
                    }
                    self.short.push_back(prev);
                    if self.dur / BEAMRULE_4_4.sixteenth
                        == new_dur / BEAMRULE_4_4.sixteenth
                    {
                        if self.dur / BEAMRULE_4_4.inner
                            == new_dur / BEAMRULE_4_4.inner
                        {
                            BeamProp::ContinueInner
                        } else {
                            BeamProp::ContinueSixteenth
                        }
                    } else {
                        BeamProp::ContinueEighth
                    }
                } else {
                    BeamProp::Flag
                };

                self.short.push_back((prop, dur, width, y));
                log!(INFO, "{:?}", self.short);
                true
            } else {
                false
            }
        } else {
            false
        };
        // Reduce remaining duration.
        self.dur = new_dur;
    }
}

impl Iterator for Beams {
    type Item = Short;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ret) = self.queued.take() {
            return Some(ret);
        }
        while let Some((prop, dur, width, y)) = self.short.pop_front() {
            match prop {
                BeamProp::None => {
                    // Start of a beam
                    let beam = if self.min_dur != 0 {
                        Some(Beam::new(self))
                    } else {
                        None
                    };
                    self.notes.push((dur, width, y, false));
                    self.min_dur = dur;
                    if let Some(beam) = beam {
                        return Some(Short::Beam(beam));
                    }
                }
                BeamProp::ContinueEighth => {
                    // If there's more than one beam, break into 2 beam groups.
                    if self.min_dur < 16 {
                        let beam = Beam::new(self);
                        self.notes.push((dur, width, y, false));
                        self.min_dur = dur;
                        return Some(Short::Beam(beam));
                    }
                    self.notes.push((dur, width, y, false));
                    self.min_dur = dur.min(self.min_dur);
                }
                BeamProp::ContinueSixteenth => {
                    // Set single beam point for 3+ beams to true
                    self.notes.push((dur, width, y, true));
                    self.min_dur = dur.min(self.min_dur);
                }
                BeamProp::ContinueInner => {
                    self.notes.push((dur, width, y, false));
                    self.min_dur = dur.min(self.min_dur);
                }
                BeamProp::Flag => {
                    let flag = Short::Flag(dur, width, y);
                    if self.min_dur != 0 {
                        let beam = Beam::new(self);
                        self.queued = Some(flag);
                        self.min_dur = 0;
                        return Some(Short::Beam(beam));
                    } else {
                        self.min_dur = 0;
                        return Some(flag);
                    }
                }
            }
        }
        if self.min_dur != 0 {
            let beam = Beam::new(self);
            self.min_dur = 0;
            Some(Short::Beam(beam))
        } else {
            None
        }
    }
}

/// Short note: A flag or a beam
pub(crate) enum Short {
    /// Flag
    Flag(u16, f32, (Vec<Pitch>, Steps)),
    /// Beam
    Beam(Beam),
}

/// A beamed group.
pub(crate) struct Beam {
    // Notes in the beamed group.
    pub(crate) notes: Vec<(u16, f32, (Pitch, Steps), bool)>,
    // Stem direction (false is down).
    pub(crate) stems_up: bool,
}

impl Beam {
    /// Create a new beam object.
    pub fn new(beams: &mut Beams) -> Self {
        // Choose stem direction of beamed group.
        let mut sum = 0i16;
        for note_i in 0..beams.notes.len() {
            let vd = beams.notes[note_i].2 .0[0].visual_distance();
            match vd.0 {
                _a if _a > 0 => sum += 1,
                _a if _a < 0 => sum -= 1,
                _ => {}
            }
        }
        let stems_up = sum < 0;
        let uses_three_beams = beams.min_dur < 8; // Less than 16th note

        // Select closest notes to the beam.
        let mut notes = vec![];
        for note in beams.notes.drain(..) {
            let one_beam = note.3 && uses_three_beams;
            // FIXME: Choose closest note to beam.
            notes.push((note.0, note.1, (note.2 .0[0], note.2 .1), one_beam));
        }

        Beam { notes, stems_up }
    }
}
