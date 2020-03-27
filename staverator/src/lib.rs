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

#![allow(clippy::blacklisted_name)] // bar is a useful musical term

mod glyph;
mod notator;
mod rhythmic_spacing;
mod svg;
mod beaming;

pub use svg::{Element, Group, Path, Rect, Use};

use notator::Notator;
use rhythmic_spacing::BarEngraver;
use beaming::{Beams, Beam, Short};

use sfff::Glyph;
use scof::{Cursor, Scof, Steps};
use std::fmt;

/// Width of one bar (measure)
const BAR_WIDTH: i32 = 2000;
/// Width of the barline.
const BARLINE_WIDTH: i32 = 36;
/// Width of a whole rest (in font units).
const WHOLE_REST_WIDTH: i32 = 230;
/// Cursor padding
const CURSOR_PADDING: i32 = 36;

/// FIXME: REMOVE - Get Bravura font paths
pub fn bravura() -> Vec<Path> {
    include!("vfont/bravura.vfont")
}

/// Get Modern font data as SVG defs.
pub fn modern() -> (sfff::SfFontMetadata, String) {
    let data: &[u8] = include_bytes!("../modern.sfff");
    let data = std::io::Cursor::new(data);
    let (meta, glyphs) = sfff::SfFontMetadata::from_buf_reader(data).unwrap();
    let glyphs = sfff::generate_defs(&glyphs);

    (meta, glyphs)
}

/// Stave lines
pub struct Stave {
    /// Number of lines on stave
    pub lines: i32,
    /// Number of steps top of stave is above middle C
    steps_middle_c: Steps,
    /// Y position (in steps).
    ypos: Steps,
}

impl Stave {
    /// A stave space
    const SPACE: i32 = 250;
    /// Half or whole step visual distance in the measure (half a stave space)
    const STEP: i32 = Self::SPACE / 2;
    /// Margin X
    const MARGIN_X: i32 = Self::SPACE;
    /// Minimum number of steps in top/bottom margins
    const MARGIN_STEPS: Steps = Steps(6);
    /// Width of stave lines (looks best if it matches BARLINE_WIDTH).
    const LINE_WIDTH: i32 = BARLINE_WIDTH;

    /// Create a new stave
    pub fn new(lines: i32, steps_middle_c: Steps, ypos: Steps) -> Self {
        Stave {
            lines,
            steps_middle_c,
            ypos,
        }
    }

    /// Get number of steps top margin is above middle C
    fn steps_top(&self, steps: Steps) -> Steps {
        let top = ((steps / 2) * 2).0 + 2; // round to nearest line
        let dflt = self.steps_middle_c + Self::MARGIN_STEPS + self.ypos;
        Steps(dflt.0.max(top))
    }

    /// Get number of steps bottom margin is above middle C
    fn steps_bottom(&self, steps: Steps) -> Steps {
        let bottom = ((steps / 2) * 2).0 - 2; // round to nearest line
        let dflt =
            self.steps_middle_c - self.height_steps() - Self::MARGIN_STEPS
                + self.ypos;
        Steps(dflt.0.min(bottom))
    }

    /// Get number of steps bottom of stave is above middle C
    fn steps_stave_bottom(&self) -> Steps {
        self.steps_middle_c - self.height_steps()
    }

    /// Get the height of the stave
    pub fn height_steps(&self) -> Steps {
        if self.lines > 0 {
            Steps(2 * (self.lines - 1))
        } else {
            Steps(0)
        }
    }

    /// Create a stave path
    pub fn path(&self, top: i32, width: i32, ofs: Steps) -> Path {
        let width = width;
        let ofs = (ofs * Stave::STEP).0;
        let mut d = String::new();
        for i in 0..self.lines {
            let x = Self::MARGIN_X;
            let y = top + Stave::SPACE * i - Stave::LINE_WIDTH / 2 + ofs;
            let line = &format!(
                "M{} {}h{}v{}h-{}v-{}z",
                x,
                y,
                width,
                Stave::LINE_WIDTH,
                width,
                Stave::LINE_WIDTH
            );
            d.push_str(line);
        }
        Path::new(None, d)
    }
}

pub struct BarElem {
    /// Stave containing the measure
    pub stave: Stave,
    /// Number of steps top margin is above middle C
    pub steps_top: Steps,
    /// Number of steps bottom margin is above middle C
    pub steps_bottom: Steps,
    /// Width of measure
    pub width: i32,
    /// SVG Elements
    pub elements: Vec<Element>,
}

impl fmt::Display for BarElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.elements {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl BarElem {
    /// Width of stems
    const STEM_WIDTH: i32 = 30;
    /// Length of stems
    const STEM_LENGTH: i32 = 7 * Stave::STEP;
    /// Maximum stem length for beamed notes on stave lines.
    const STEM_LENGTH_LINE: i32 = Self::STEM_LENGTH - (Stave::STEP / 2);
    /// FIXME: Minimum Shortened Stem Length For Notes On Ledger Lines
    const _STEM_LENGTH_LEDGER: i32 = 5 * Stave::STEP;
    /// Minimum Shortened Stem Length For Notes On Stave
    const STEM_LENGTH_SHORT: i32 = 6 * Stave::STEP;
    /// Width of note head
    const HEAD_WIDTH: i32 = 266;

    /// Create a new bar element
    pub fn new(stave: Stave, high: Steps, low: Steps) -> Self {
        let steps_top = stave.steps_top(high);
        let steps_bottom = stave.steps_bottom(low);
        let width = 0;
        let elements = vec![];
        Self {
            stave,
            steps_top,
            steps_bottom,
            width,
            elements,
        }
    }

    /// Add markings to this measure.
    ///
    /// - `scof`: The score.
    /// - `curs`: Cursor of measure.
    pub fn add_markings(
        &mut self,
        scof: &Scof,
        cursor: &Cursor,
        curs: &mut Cursor,
    ) -> Option<(i32, i32, i32, i32)> {
        let reset_cursor = curs.clone();

        // Make notators for each stave.
        let mut notators = vec![];
        for chan in 0..scof.movement[0].bar[0].chan.len() as u16 {
            *curs = reset_cursor.chan(chan);
            notators.push(Notator::new(scof, cursor.clone(), curs.clone()));
        }

        // Engrave the music.
        let (width, rect) = BarEngraver::new(self, &mut notators).engrave();
        self.width += width;
        rect
    }

    /// Get the Y offset of a step value
    fn offset_y(&self, steps: Steps) -> i32 {
        debug_assert!(steps.0 <= self.steps_top.0);
        ((self.steps_top - steps) * Stave::STEP).0
    }

    /// Get the full height
    fn height(&self) -> i32 {
        ((self.steps_top - self.steps_bottom) * Stave::STEP).0
    }

    /// Get the middle of the stave y position
    fn middle(&self) -> i32 {
        let steps = self.stave.steps_middle_c - self.stave.height_steps() / 2;
        self.offset_y(steps)
    }

    /// Add a barline to stave
    fn add_barline(&mut self, x: i32, ofs: Steps) {
        let width = BARLINE_WIDTH;
        let ofs = (ofs * Stave::STEP).0;
        let y = self.offset_y(self.stave.steps_middle_c) + ofs;
        let y_bottom = self.offset_y(self.stave.steps_stave_bottom()) + ofs;
        let height = y_bottom - y;
        let rect = Rect::new(
            x + Stave::MARGIN_X,
            y,
            width,
            height,
            None,
            None,
            None,
        );
        self.elements.push(Element::Rect(rect));
    }

    /// Get Y position from steps and offset
    fn y_from_steps(&self, steps: Steps, ofs: Steps) -> i32 {
        let ofs = (ofs * Stave::STEP).0;
        let y = self.offset_y(steps);

        y + ofs
    }

    /// Add elements for flag and stem.
    fn add_flag(&mut self, dur: u16, offset: f32, y: Steps, y_offset: Steps) {
        let y = self.y_from_steps(y, y_offset);
        let flag_glyph = glyph::flag_duration(dur, y > self.middle()).unwrap();
        let x = Stave::MARGIN_X
            + self.width
            + ((offset * BAR_WIDTH as f32) as i32);

        let (ofsx, ofsy) = if y > self.middle() {
            (Self::HEAD_WIDTH, -(Self::STEM_LENGTH))
        } else {
            (0, Self::STEM_LENGTH)
        };

        self.add_use(flag_glyph, x + ofsx, y + ofsy);
        self.add_stem(x, y, Self::STEM_LENGTH);
    }

    /// Add beam element.
    fn add_beam(&mut self, beam: Beam) {
        let thickness = Stave::STEP;
        let (add_stem, ofsx, ofsy): (fn(&mut BarElem, i32, i32, i32), _, _) = if beam.stems_up {
            (Self::add_stem_up, Self::HEAD_WIDTH, -Self::STEM_LENGTH)
        } else {
            (Self::add_stem_down, 0i32, Self::STEM_LENGTH - thickness)
        };

        let mut d = String::new();
        cala::info!("ADD_BEAM {} notes", beam.notes.len());
        let mut old_x = None;
        for note_i in 0..beam.notes.len() {
            let (y, y_offset) = beam.notes[note_i].2;
            let y = self.y_from_steps(y.visual_distance(), y_offset);
            let x = Stave::MARGIN_X
                + self.width
                + ((beam.notes[note_i].1 * BAR_WIDTH as f32) as i32);

            add_stem(self, x, y, Self::STEM_LENGTH);

            if let Some(old_x) = old_x {
                let diff: i32 = x - old_x;

                let mut count = match beam.notes[note_i].0 {
                    1 => 5, // 128th note beams
                    2..=3 => 4, // 64th note beams
                    4..=7 => 3, // 32nd note beams
                    8..=15 => 2, // 16th note beams
                    16..=31 => 1, // 8th note beams
                    a => panic!("Invalid {}", a),
                };

                if beam.notes[note_i].3 {
                    count = count.min(1);
                }

                for i in 0..count {
                    d.push_str(&format!("M{} {}l{} {}l{} {}l{} {}z", x + ofsx, y + ofsy - (i * 3 * Stave::STEP) / 2, -diff, 0, 0, thickness, diff, 0));
                }
            }
            old_x = Some(x);
        }
        self.elements.push(Element::Path(Path::new(None, d)));
    }

    /// Add stems and either flags or beam elements for short notes.
    fn add_flags_and_beams(
        &mut self,
        beams: Beams,
    ) {
        for short in beams {
            match short {
                Short::Flag(dur, offset, (pitches, y_offset)) => {
                    let pitch = pitches[0]; // FIXME: Use closest to beam/flag.
                    self.add_flag(dur, offset, pitch.visual_distance(), y_offset);
                }
                Short::Beam(beam) => {
                    self.add_beam(beam)
                }
            }
        }
    }

    /// Add elements for a note
    fn add_pitch(
        &mut self,
        dur: u16,
        offset: f32,
        steps: Steps,
        y: i32,
    ) {
        let x = Stave::MARGIN_X
            + self.width
            + ((offset * BAR_WIDTH as f32) as i32);

        let cp = glyph::notehead_duration(dur);
        self.add_use(cp, x, y);
        // Only draw stem if not a whole note or double whole note (breve) or
        // Shorter than quarter note.
        match dur {
            1..=31 | 128..=511 => {}
            _ => self.add_stem(x, y, Self::STEM_LENGTH),
        }

        // Draw Ledger Lines if below or above stave.
        let mut head_width = Self::HEAD_WIDTH;
        if dur >= 128 {
            // Whole note, breve, and longa all have wide noteheads.
            head_width += Self::HEAD_WIDTH / 2;
        }
        let dir_step = if steps.0 > 0 { 1 } else { -1 } * Stave::STEP;
        let yyy = steps.0.abs();
        let mut count = if yyy % 2 == 0 { 0 } else { 1 };
        for _ in (6..yyy + 1).step_by(2) {
            let rect = Rect::new(
                x - ((Self::HEAD_WIDTH - (Self::STEM_WIDTH / 2)) / 2),
                y - (Stave::LINE_WIDTH / 2) + count * dir_step,
                Self::HEAD_WIDTH + head_width,
                Stave::LINE_WIDTH,
                None,
                None,
                None,
            );
            self.elements.push(Element::Rect(rect));
            count += 2;
        }
    }

    /// Add a stem
    fn add_stem(&mut self, x: i32, y: i32, stem_length: i32) {
        if y > self.middle() {
            self.add_stem_up(x, y, stem_length);
        } else {
            self.add_stem_down(x, y, stem_length);
        }
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32, stem_length: i32) {
        // FIXME: stem should always reach the center line of the stave
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect =
            Rect::new(x, y, Self::STEM_WIDTH, stem_length, rx, ry, None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32, stem_length: i32) {
        // FIXME: stem should always reach the center line of the stave
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(
            x + Self::HEAD_WIDTH,
            y - stem_length,
            Self::STEM_WIDTH,
            stem_length,
            rx,
            ry,
            None,
        );
        self.elements.push(Element::Rect(rect));
    }

    /// Add `use` element for a whole measure rest
    fn add_measure_rest(&mut self, width: f32, y: Steps) {
        let x = Stave::MARGIN_X
            + ((width * BAR_WIDTH as f32) as i32 - WHOLE_REST_WIDTH) / 2;
        let y = self.middle() + ((y - Steps(2)) * Stave::STEP).0;
        self.add_use(Glyph::Rest1, x, y);
    }

    /// Add `use` element for a rest.
    fn add_rest(&mut self, glyph: Glyph, offset: f32, ofs: Steps) {
        let x = Stave::MARGIN_X
            + self.width
            + ((offset * BAR_WIDTH as f32) as i32);
        let ofs = (ofs * Stave::STEP).0;
        let mut y = self.middle() + ofs;
        // Position whole rest glyph up 1 stave space.
        if glyph == Glyph::Rest1 {
            y -= Stave::SPACE;
        }
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: Glyph, x: i32, y: i32) {
        self.elements
            .push(Element::Use(Use::new(x, y, glyph.into())));
    }

    /// Add clef
    pub fn add_clefs(&mut self, scof: &Scof) {
        for i in 0..scof.movement[0].bar[0].chan.len() as i32 {
            let ymargin =
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP;
            self.add_use(
                Glyph::ClefC,
                Stave::MARGIN_X + 150,
                self.middle() + ymargin * i,
            );
        }
        self.width += 1000;
    }

    /// Add time signature
    pub fn add_times(&mut self, scof: &Scof) {
        for i in 0..scof.movement[0].bar[0].chan.len() as i32 {
            let ymargin =
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP;
            // width=421
            self.add_use(
                Glyph::TimeSig3,
                Stave::MARGIN_X + self.width + 50,
                self.middle() - Stave::SPACE + ymargin * i,
            );
            // width=470
            self.add_use(
                Glyph::TimeSig4,
                Stave::MARGIN_X + self.width + 50 - ((470 - 421) / 2),
                self.middle() + Stave::SPACE + ymargin * i,
            );
        }

        self.width += 640;
    }

    /// Add clef & time signature.
    pub fn add_signatures(&mut self, _scof: &Scof) {
        //self.add_clefs(_scof);
        //self.add_times(_scof);
    }
}

#[cfg(test)]
mod tests {}
