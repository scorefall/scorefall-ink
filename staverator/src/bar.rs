// ScoreFall Ink - Music Composition Software
//
// Copyright (C) 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
// Copyright (C) 2019-2026 Doug P. Lau
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
//
use std::fmt;

use devout::{Tag, log};
use hatmil::{
    Page,
    svg::{Path, Rect, Use},
};
use scof::{Cursor, Scof, Steps};
use sfff::{Glyph, SfFontMetadata};

use crate::{
    beaming::{Beam, Beams, Short},
    glyph, notator,
    notehead::{self, Notehead},
    rhythmic_spacing::BarEngraver,
    stave::Stave,
};

const INFO: Tag = Tag::new("Staverator");

/// The number of units per stave space.
pub const STAVE_SPACE: i32 = 100;
/// Width of one bar (measure)
pub const BAR_WIDTH: i32 = 8 * STAVE_SPACE;
/// Width of a whole rest (in font units).
pub const WHOLE_REST_WIDTH: i32 = 230;

/// Bar element
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
    pub(crate) elements: Vec<String>,
}

impl fmt::Display for BarElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.elements {
            write!(f, "{elem}")?;
        }
        Ok(())
    }
}

impl BarElem {
    /// Length of stems
    const STEM_LENGTH: i32 = 7 * Stave::STEP;
    /// FIXME: Minimum Shortened Stem Length For Notes On Ledger Lines
    const _STEM_LENGTH_LEDGER: i32 = 5 * Stave::STEP;
    /// Maximum stem length for beamed notes on stave lines.
    const _STEM_LENGTH_LINE: i32 = Self::STEM_LENGTH - (Stave::STEP / 2);
    /// Minimum Shortened Stem Length For Notes On Stave
    const _STEM_LENGTH_SHORT: i32 = 6 * Stave::STEP;

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
    /// - `cursor`: Current cursor position.
    /// - `measure`: Measure of bar.
    pub fn add_markings(
        &mut self,
        meta: &SfFontMetadata,
        scof: &Scof,
        cursor: &Cursor,
        measure: u16,
    ) {
        let mut curs = Cursor::new(
            0, /* mvmt */
            measure, 0, /* i chan */
            0, /* marking */
        );
        let reset_cursor = curs;

        // Make voicings for each stave.
        let mut stave_voicing = vec![];
        let mut stave_cursor = vec![];
        for chan in 0..scof.movement[0].bar[0].chan.len() as u16 {
            curs = reset_cursor.chan(chan);
            let notes = notator::notate(scof, *cursor, curs);
            stave_voicing.push(notes);
            stave_cursor.push(*cursor == curs);
        }

        // Engrave the music.
        let engraver = BarEngraver::new(meta, stave_voicing, stave_cursor);
        self.width += self.engrave(engraver);
    }

    /// Get the Y offset of a step value
    pub(crate) fn offset_y(&self, steps: Steps) -> i32 {
        debug_assert!(steps.0 <= self.steps_top.0);
        ((self.steps_top - steps) * Stave::STEP).0
    }

    /// Get the full height
    pub(crate) fn height(&self) -> i32 {
        ((self.steps_top - self.steps_bottom) * Stave::STEP).0
    }

    /// Get the middle of the stave y position
    fn middle(&self) -> i32 {
        let steps = self.stave.steps_middle_c - self.stave.height_steps() / 2;
        self.offset_y(steps)
    }

    /// Add a barline to stave
    pub(crate) fn add_barline(
        &mut self,
        meta: &SfFontMetadata,
        x: i32,
        ofs: Steps,
    ) {
        let width = meta.barline_thickness;
        let ofs = (ofs * Stave::STEP).0;
        let y = self.offset_y(self.stave.steps_middle_c) + ofs;
        let y_bottom = self.offset_y(self.stave.steps_stave_bottom()) + ofs;
        let height = y_bottom - y;
        let mut page = Page::new();
        page.frag::<Rect>().x(x).y(y).width(width).height(height);
        self.elements.push(String::from(page));
    }

    /// Get Y position from steps and offset
    pub(crate) fn y_from_steps(&self, steps: Steps, ofs: Steps) -> i32 {
        let ofs = (ofs * Stave::STEP).0;
        let y = self.offset_y(steps);

        y + ofs
    }

    /// Add elements for flag and stem.
    fn add_flag(
        &mut self,
        meta: &SfFontMetadata,
        dur: u16,
        offset: f32,
        y: Steps,
        y_offset: Steps,
    ) {
        let y = self.y_from_steps(y, y_offset);
        let flag_glyph = glyph::flag_duration(dur, y > self.middle()).unwrap();
        let x = self.width + ((offset * BAR_WIDTH as f32) as i32);
        let [left, right] = notehead::stems(Notehead::Normal, meta, dur);

        if y > self.middle() {
            // Right Stem
            let ofsx = right[0] - meta.stem_thickness;
            let ofsy = -Self::STEM_LENGTH;

            self.add_use(flag_glyph, x + ofsx, y + ofsy);
            self.add_stem2(meta, x + ofsx, y + ofsy, Self::STEM_LENGTH);
        } else {
            // Left Stem
            let ofsx = left[0];
            let ofsy = Self::STEM_LENGTH;

            self.add_use(flag_glyph, x + ofsx, y + ofsy);
            self.add_stem2(meta, x + ofsx, y, Self::STEM_LENGTH);
        };
    }

    /// Add beam element.
    fn add_beam(&mut self, meta: &SfFontMetadata, beam: Beam) {
        let thickness = Stave::STEP;
        let [left, right] = notehead::stems(Notehead::Normal, meta, 32);
        let (ofsx, ofsy, beamy) = if beam.stems_up {
            (right[0] - meta.stem_thickness, -Self::STEM_LENGTH, 0)
        } else {
            (left[0], 0, Self::STEM_LENGTH - thickness)
        };

        let mut d = Path::def_builder();
        log!(INFO, "ADD_BEAM {} notes", beam.notes.len());
        let mut old_x = None;
        for note_i in 0..beam.notes.len() {
            let (y, y_offset) = beam.notes[note_i].2;
            let y = self.y_from_steps(y.visual_distance(), y_offset);
            let x =
                self.width + ((beam.notes[note_i].1 * BAR_WIDTH as f32) as i32);

            self.add_stem2(meta, x + ofsx, y + ofsy, Self::STEM_LENGTH);

            if let Some(old_x) = old_x {
                let diff: i32 = x - old_x;

                let mut count = match beam.notes[note_i].0 {
                    1 => 5,       // 128th note beams
                    2..=3 => 4,   // 64th note beams
                    4..=7 => 3,   // 32nd note beams
                    8..=15 => 2,  // 16th note beams
                    16..=31 => 1, // 8th note beams
                    a => panic!("Invalid {}", a),
                };

                if beam.notes[note_i].3 {
                    count = count.min(1);
                }

                let beam_distance =
                    if beam.stems_up { -1 } else { 1 } * (3 * Stave::STEP) / 2;
                for i in 0..count {
                    let x0 = x + ofsx + (meta.stem_thickness / 2);
                    let x1 = x0 - diff;
                    let y0 = y + ofsy + beamy - i * beam_distance;
                    let y1 = y0 + thickness;
                    d.move_to((x0, y0));
                    d.line((x1, y0));
                    d.line((x1, y1));
                    d.line((x0, y1));
                    d.close();
                }
            }
            old_x = Some(x);
        }
        let mut page = Page::new();
        page.frag::<Path>().d(String::from(d));
        self.elements.push(String::from(page));
    }

    /// Add stems and either flags or beam elements for short notes.
    pub(crate) fn add_flags_and_beams(
        &mut self,
        meta: &SfFontMetadata,
        beams: Beams,
    ) {
        for short in beams {
            match short {
                Short::Flag(dur, offset, (pitches, y_offset)) => {
                    let pitch = pitches[0]; // FIXME: Use closest to beam/flag.
                    self.add_flag(
                        meta,
                        dur,
                        offset,
                        pitch.visual_distance(),
                        y_offset,
                    );
                }
                Short::Beam(beam) => self.add_beam(meta, beam),
            }
        }
    }

    /// Add elements for a note
    pub(crate) fn add_pitch(
        &mut self,
        meta: &SfFontMetadata,
        dur: u16,
        offset: f32,
        steps: Steps,
        y: i32,
    ) {
        let x = self.width + ((offset * BAR_WIDTH as f32) as i32);

        let cp = notehead::duration(dur);
        self.add_use(cp, x, y);
        // Only draw stem if not a whole note or double whole note (breve) or
        // Shorter than quarter note.
        match dur {
            1..=31 | 128..=511 => {}
            _ => {
                let [left, right] =
                    notehead::stems(Notehead::Normal, meta, dur);
                let (ofsx, ofsy) = if y > self.middle() {
                    (right[0] - meta.stem_thickness, -Self::STEM_LENGTH)
                } else {
                    (left[0], 0)
                };
                self.add_stem2(meta, x + ofsx, y + ofsy, Self::STEM_LENGTH)
            }
        }

        // Draw Ledger Lines if below or above stave.
        let head_width = notehead::width(Notehead::Normal, meta, dur);
        let dir_step = if steps.0 > 0 { 1 } else { -1 } * Stave::STEP;
        let yyy = steps.0.abs();
        let mut count = if yyy % 2 == 0 { 0 } else { 1 };
        for _ in (6..yyy + 1).step_by(2) {
            let x =
                x - (meta.ledger_line_extension - (meta.stem_thickness / 4));
            let y = y - (meta.stave_line_thickness / 2) + count * dir_step;
            let width = head_width + meta.ledger_line_extension * 2;
            let height = meta.stave_line_thickness;
            let mut page = Page::new();
            page.frag::<Rect>().x(x).y(y).width(width).height(height);
            self.elements.push(String::from(page));
            count += 2;
        }
    }

    /// Add a stem FIXME: Replace add_stem with this.
    fn add_stem2(
        &mut self,
        meta: &SfFontMetadata,
        x: i32,
        y: i32,
        stem_length: i32,
    ) {
        let width = meta.stem_thickness;
        let height = stem_length;
        let rx = meta.stem_thickness / 2;
        let ry = meta.stem_thickness;
        let mut page = Page::new();
        page.frag::<Rect>()
            .x(x)
            .y(y)
            .width(width)
            .height(height)
            .rx(rx)
            .ry(ry);
        self.elements.push(String::from(page));
    }

    /// Add `use` element for a whole measure rest
    pub(crate) fn add_measure_rest(&mut self, width: f32, y: Steps) {
        let x = ((width * BAR_WIDTH as f32) as i32 - WHOLE_REST_WIDTH) / 2;
        let y = self.middle() + ((y - Steps(2)) * Stave::STEP).0;
        self.add_use(Glyph::Rest1, x, y);
    }

    /// Add `use` element for a rest.
    pub(crate) fn add_rest(&mut self, glyph: Glyph, offset: f32, ofs: Steps) {
        let x = self.width + ((offset * BAR_WIDTH as f32) as i32);
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
        let mut page = Page::new();
        page.frag::<Use>()
            .x(x)
            .y(y)
            .href(format!("#{:x}", u16::from(glyph)));
        self.elements.push(String::from(page));
    }

    /// Add clefs
    #[expect(dead_code)]
    pub(crate) fn add_clefs(&mut self, scof: &Scof) {
        for i in 0..scof.movement[0].bar[0].chan.len() as i32 {
            let ymargin =
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP;
            self.add_use(Glyph::ClefC, 150, self.middle() + ymargin * i);
        }
        self.width += 1000;
    }

    /// Add time signature
    #[expect(dead_code)]
    pub(crate) fn add_times(&mut self, scof: &Scof) {
        for i in 0..scof.movement[0].bar[0].chan.len() as i32 {
            let ymargin =
                (self.stave.height_steps() + Steps(12)).0 * Stave::STEP;
            // width=421
            self.add_use(
                Glyph::TimeSig3,
                self.width + 50,
                self.middle() - Stave::SPACE + ymargin * i,
            );
            // width=470
            self.add_use(
                Glyph::TimeSig4,
                self.width + 50 - ((470 - 421) / 2),
                self.middle() + Stave::SPACE + ymargin * i,
            );
        }

        self.width += 640;
    }

    /// Add clef & time signature.
    #[expect(dead_code)]
    pub(crate) fn add_signatures(&mut self, _scof: &Scof) {
        //self.add_clefs(_scof);
        //self.add_times(_scof);
    }
}

#[cfg(test)]
mod tests {}
