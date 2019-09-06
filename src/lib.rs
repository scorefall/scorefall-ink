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
use scof::Marking;

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

    /// Step up or down within the key.
    fn move_step(&mut self, up: bool) {
        let create = (scof::PitchClass {
            name: scof::PitchName::C,
            accidental: None,
        }, 4);

        if let Some(mark) = self.scof.marking(self.bar, self.chan, self.curs) {
            match mark {
                Marking::Dynamic(_) => {/*Do nothing*/},
                Marking::GraceInto(note) => {
                    self.scof.set_pitch(self.bar, self.chan, self.curs, if up { note.step_up(create) } else { note.step_down(create) }.pitch.unwrap())
                },
                Marking::GraceOutOf(note) => {
                    self.scof.set_pitch(self.bar, self.chan, self.curs, if up { note.step_up(create) } else { note.step_down(create) }.pitch.unwrap())
                },
                Marking::Note(note) => {
                    self.scof.set_pitch(self.bar, self.chan, self.curs, if up { note.step_up(create) } else { note.step_down(create) }.pitch.unwrap())
                },
                Marking::Breath => {/*Do nothing*/},
                Marking::CaesuraShort => {/*Do nothing*/},
                Marking::CaesuraLong => {/*Do nothing*/},
                Marking::Cresc => {/*Do nothing*/},
                Marking::Dim => {/*Do nothing*/},
                Marking::Pizz => {/*Do nothing*/},
                Marking::Arco => {/*Do nothing*/},
                Marking::Mute => {/*Do nothing*/},
                Marking::Open => {/*Do nothing*/},
                Marking::Repeat => {/*Do nothing*/},
            }
        } else {
            // Shouldn't happen, do nothing.
        }
    }

    /// Move a note down 1 step within the key.
    pub fn down_step(&mut self) {
        self.move_step(false);
    }

    /// Move a note up 1 step within the key.
    pub fn up_step(&mut self) {
        self.move_step(true);
    }
}
