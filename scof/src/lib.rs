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

mod articulation;
mod chan;
mod cursor;
mod fraction;
mod marking;
mod measure;
mod movement;
pub mod note;
mod score;
mod synth;

pub use crate::{
    articulation::Articulation,
    chan::{Chan, Channel},
    cursor::Cursor,
    fraction::{Fraction, IsZero},
    marking::{Dynamic, Marking, Repeat, Sig},
    measure::{Bar, Measure},
    movement::Movement,
    note::{
        Note, Pitch, PitchAccidental, PitchClass, PitchName, PitchOctave, Steps,
    },
    score::Scof,
    synth::{Instrument, Synth},
};
