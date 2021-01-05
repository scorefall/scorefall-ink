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

use scof::{Cursor, Fraction, Marking, Note, Pitch, Scof};

/// This is the entire program context.
pub struct Program {
    /// The save file.
    pub scof: Scof,
    /// Current cursor
    pub cursor: Cursor,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            scof: Scof::default(),
            cursor: Cursor::default(),
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
        self.cursor.left(&self.scof);
    }

    /// Move cursor forward.
    pub fn right(&mut self) {
        self.cursor.right(&self.scof);
        // If measure doesn't exist, make a new one.
        if self.scof.marking_is_empty(&self.cursor) {
            self.scof.new_measure();
        }
    }

    /// Step up or down within the key.
    fn move_step(&mut self, up: bool, gran: u8) {
        let step_up_fn = match gran {
            0 => Note::step_up,
            1 => Note::half_step_up,
            2 => Note::quarter_step_up,
            _ => unreachable!(),
        };
        let step_down_fn = match gran {
            0 => Note::step_down,
            1 => Note::half_step_down,
            2 => Note::quarter_step_down,
            _ => unreachable!(),
        };

        let create = Pitch(
            scof::PitchClass {
                name: scof::PitchName::C,
                accidental: None,
            },
            scof::PitchOctave::Octave4,
        );

        if let Some(mark) = self.scof.marking(&self.cursor).cloned() {
            match mark {
                Marking::Dynamic(_) => { /*Do nothing*/ }
                Marking::GraceInto(note) => self.scof.set_pitch(
                    &self.cursor,
                    0,
                    if up {
                        step_up_fn(&note, 0, create)
                    } else {
                        step_down_fn(&note, 0, create)
                    }
                    .pitch[0],
                ),
                Marking::GraceOutOf(note) => self.scof.set_pitch(
                    &self.cursor,
                    0,
                    if up {
                        step_up_fn(&note, 0, create)
                    } else {
                        step_down_fn(&note, 0, create)
                    }
                    .pitch[0],
                ),
                Marking::Note(note) => self.scof.set_pitch(
                    &self.cursor,
                    0,
                    if up {
                        step_up_fn(&note, 0, create)
                    } else {
                        step_down_fn(&note, 0, create)
                    }
                    .pitch[0],
                ),
                Marking::Breath => { /*Do nothing*/ }
                Marking::CaesuraShort => { /*Do nothing*/ }
                Marking::CaesuraLong => { /*Do nothing*/ }
                Marking::Cresc => { /*Do nothing*/ }
                Marking::Dim => { /*Do nothing*/ }
                Marking::Pizz => { /*Do nothing*/ }
                Marking::Arco => { /*Do nothing*/ }
                Marking::Mute => { /*Do nothing*/ }
                Marking::Open => { /*Do nothing*/ }
                Marking::Repeat => { /*Do nothing*/ }
            }
        } else {
            self.scof.set_whole_pitch(&self.cursor);
        }
    }

    /// Move a note down 1 step within the key.
    pub fn down_step(&mut self) {
        self.move_step(false, 0);
    }

    /// Move a note up 1 step within the key.
    pub fn up_step(&mut self) {
        self.move_step(true, 0);
    }

    /// Move a note down 1 step within the key.
    pub fn down_half_step(&mut self) {
        self.move_step(false, 1);
    }

    /// Move a note up 1 step within the key.
    pub fn up_half_step(&mut self) {
        self.move_step(true, 1);
    }

    /// Move a note down 1 step within the key.
    pub fn down_quarter_step(&mut self) {
        self.move_step(false, 2);
    }

    /// Move a note up 1 step within the key.
    pub fn up_quarter_step(&mut self) {
        self.move_step(true, 2);
    }

    /// Set duration of a note.
    pub fn set_dur(&mut self, dur: Fraction) {
        if let Some(mark) = self.scof.marking(&self.cursor) {
            match mark {
                Marking::Dynamic(_) => { /*Do nothing*/ }
                Marking::GraceInto(_note) => {
                    self.scof.set_duration(&self.cursor, dur)
                }
                Marking::GraceOutOf(_note) => {
                    self.scof.set_duration(&self.cursor, dur)
                }
                Marking::Note(_note) => {
                    self.scof.set_duration(&self.cursor, dur)
                }
                Marking::Breath => { /*Do nothing*/ }
                Marking::CaesuraShort => { /*Do nothing*/ }
                Marking::CaesuraLong => { /*Do nothing*/ }
                Marking::Cresc => { /*Do nothing*/ }
                Marking::Dim => { /*Do nothing*/ }
                Marking::Pizz => { /*Do nothing*/ }
                Marking::Arco => { /*Do nothing*/ }
                Marking::Mute => { /*Do nothing*/ }
                Marking::Open => { /*Do nothing*/ }
                Marking::Repeat => { /*Do nothing*/ }
            }
        } else {
            self.scof.set_whole_duration(&self.cursor, dur);
        }
    }

    /// Set duration of a note to tuplet.
    pub fn tuplet(&mut self) {
        // FIXME
    }

    /// Set duration of note to dotted.
    pub fn dotted(&mut self) {
        // FIXME
    }
}
