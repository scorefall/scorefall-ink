// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019-2020 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
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

#![allow(clippy::blacklisted_name)] // bar is a useful musical term

mod glyph;
mod notator;
mod rhythmic_spacing;
mod svg;

pub use glyph::GlyphId;
pub use svg::{Element, Group, Path, Rect, Use};

use notator::Notator;
use rhythmic_spacing::BarEngraver;

use scof::{Cursor, Scof, Steps};
use std::fmt;

/// Width of one bar (measure)
const BAR_WIDTH: i32 = 3200;
/// Width of the barline.
const BARLINE_WIDTH: i32 = 36;
/// Space before each note.
const NOTE_MARGIN: i32 = BARLINE_WIDTH; // 250;
/// Width of a whole rest (in font units).
const WHOLE_REST_WIDTH: i32 = 230;

/// Get Bravura font paths
pub fn bravura() -> Vec<Path> {
    include!("vfont/bravura.vfont")
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
    /// A half or whole step visual distance in the measure.
    const STEP_DY: i32 = 125;
    /// Margin X
    const MARGIN_X: i32 = BARLINE_WIDTH;
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
        let width = width + (BARLINE_WIDTH / 2);
        let ofs = (ofs * Stave::STEP_DY).0;
        let mut d = String::new();
        for i in 0..self.lines {
            let x = Self::MARGIN_X - (BARLINE_WIDTH / 2);
            let y =
                top + Stave::STEP_DY * (i * 2) - Stave::LINE_WIDTH / 2 + ofs;
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
    const STEM_LENGTH: i32 = 7 * Stave::STEP_DY;
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
        ((self.steps_top - steps) * Stave::STEP_DY).0
    }

    /// Get the full height
    fn height(&self) -> i32 {
        ((self.steps_top - self.steps_bottom) * Stave::STEP_DY).0
    }

    /// Get the middle of the stave y position
    fn middle(&self) -> i32 {
        let steps = self.stave.steps_middle_c - self.stave.height_steps() / 2;
        self.offset_y(steps)
    }

    /// Add a barline to stave
    fn add_barline(&mut self, x: i32, ofs: Steps) {
        let width = BARLINE_WIDTH;
        let ofs = (ofs * Stave::STEP_DY).0;
        let y = self.offset_y(self.stave.steps_middle_c) + ofs;
        let y_bottom = self.offset_y(self.stave.steps_stave_bottom()) + ofs;
        let height = y_bottom - y;
        let rect = Rect::new(
            x + (Stave::MARGIN_X - BARLINE_WIDTH),
            y,
            width,
            height,
            None,
            None,
            None,
        );
        self.elements.push(Element::Rect(rect));
    }

    /// Add elements for a note
    fn add_pitch(
        &mut self,
        dur: u16,
        offset: f32,
        steps: scof::Steps,
        ofs: Steps,
    ) {
        let x = (Stave::MARGIN_X - BARLINE_WIDTH)
            + NOTE_MARGIN
            + self.width
            + ((offset * BAR_WIDTH as f32) as i32);
        let ofs = (ofs * Stave::STEP_DY).0;
        let y = self.offset_y(steps);
        let cp = GlyphId::notehead_duration(dur);
        self.add_use(cp, x, y + ofs);
        // Only draw stem if not a whole note or double whole note (breve).
        match dur {
            128..=511 => {}
            _ => self.add_stem(x, y, ofs),
        }
        // Draw flag if 8th note or shorter.
        if let Some(flag_glyph) = GlyphId::flag_duration(dur, y > self.middle())
        {
            let (ofsx, ofsy) = if y > self.middle() {
                (Self::HEAD_WIDTH, -(Self::STEM_LENGTH))
            } else {
                (0, Self::STEM_LENGTH)
            };

            self.add_use(flag_glyph, x + ofsx, y + ofsy + ofs);
        }
        // Draw Ledger Lines if below or above stave.
        let head_width = if dur >= 128 {
            // Whole note, breve, and longa all have wide noteheads.
            Self::HEAD_WIDTH + (Self::HEAD_WIDTH / 2)
        } else {
            Self::HEAD_WIDTH
        };
        let yyy = steps.0; // - self.middle_steps();
        if yyy > 0 {
            let mut count = if yyy % 2 == 0 { 0 } else { 1 };
            for _ in (6..yyy + 1).step_by(2) {
                let rect = Rect::new(
                    x - ((Self::HEAD_WIDTH - (Self::STEM_WIDTH / 2)) / 2),
                    ofs + y - (Stave::LINE_WIDTH / 2)
                        + (count * Stave::STEP_DY),
                    Self::HEAD_WIDTH + head_width,
                    Stave::LINE_WIDTH,
                    None,
                    None,
                    None,
                );
                self.elements.push(Element::Rect(rect));
                count += 2;
            }
        } else {
            let yyy = -yyy;
            let mut count = if yyy % 2 == 0 { 0 } else { 1 };
            for _ in (6..yyy + 1).step_by(2) {
                let rect = Rect::new(
                    x - ((Self::HEAD_WIDTH - (Self::STEM_WIDTH / 2)) / 2),
                    ofs + y
                        - (Stave::LINE_WIDTH / 2)
                        - (count * Stave::STEP_DY),
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
    }

    /// Add a stem
    fn add_stem(&mut self, x: i32, y: i32, ofs: i32) {
        if y > self.middle() {
            self.add_stem_up(x, y + ofs);
        } else {
            self.add_stem_down(x, y + ofs);
        }
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the stave
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect =
            Rect::new(x, y, Self::STEM_WIDTH, Self::STEM_LENGTH, rx, ry, None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the stave
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(
            x + Self::HEAD_WIDTH,
            y - Self::STEM_LENGTH,
            Self::STEM_WIDTH,
            Self::STEM_LENGTH,
            rx,
            ry,
            None,
        );
        self.elements.push(Element::Rect(rect));
    }

    /// Add `use` element for a whole measure rest
    fn add_measure_rest(&mut self, width: f32, y: Steps) {
        let x = (Stave::MARGIN_X - BARLINE_WIDTH)
            + ((width * BAR_WIDTH as f32) as i32 - WHOLE_REST_WIDTH) / 2;
        let y = self.middle() + ((y - Steps(2)) * Stave::STEP_DY).0;
        self.add_use(GlyphId::Rest1, x, y);
    }

    /// Add `use` element for a rest.
    fn add_rest(&mut self, glyph: GlyphId, offset: f32, ofs: Steps) {
        let x = (Stave::MARGIN_X - BARLINE_WIDTH)
            + NOTE_MARGIN
            + self.width
            + ((offset * BAR_WIDTH as f32) as i32);
        let ofs = (ofs * Stave::STEP_DY).0;
        let mut y = self.middle() + ofs;
        // Position whole rest glyph up 2 steps.
        if glyph == GlyphId::Rest1 {
            y -= Stave::STEP_DY * 2;
        }
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: GlyphId, x: i32, y: i32) {
        self.elements
            .push(Element::Use(Use::new(x, y, glyph.into())));
    }

    /// Add clef
    pub fn add_clefs(&mut self, scof: &Scof) {
        for i in 0..scof.movement[0].bar[0].chan.len() as i32 {
            let ymargin =
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP_DY;
            self.add_use(
                GlyphId::ClefC,
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
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP_DY;
            // width=421
            self.add_use(
                GlyphId::TimeSig3,
                Stave::MARGIN_X + self.width + 50,
                self.middle() - Stave::STEP_DY * 2 + ymargin * i,
            );
            // width=470
            self.add_use(
                GlyphId::TimeSig4,
                Stave::MARGIN_X + self.width + 50 - ((470 - 421) / 2),
                self.middle() + Stave::STEP_DY * 2 + ymargin * i,
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
