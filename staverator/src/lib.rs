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

mod glyph;
mod svg;
mod notator;

pub use glyph::GlyphId;
pub use svg::{Element, Group, Path, Rect, Use};

use notator::Notator;

use scof::{Cursor, Fraction, Marking, Note, Scof, Steps};
use std::fmt;

/// Width of one bar (measure)
const BAR_WIDTH: i32 = 3200;
/// Width of the barline.
const BARLINE_WIDTH: i32 = 36;
/// Space before each note.
const NOTE_MARGIN: i32 = 250;
/// Color of cursor
const CURSOR_COLOR: u32 = 0xFF9AF0;
/// Width of a whole rest (in font units).
const WHOLE_REST_WIDTH: i32 = 230;

/// Get Bravura font paths
pub fn bravura() -> Vec<Path> {
    include!("vfont/bravura.vfont")
}

/// Staff lines
pub struct Staff {
    /// Number of lines on staff
    pub lines: i32,
    /// Number of steps top of staff is above middle C
    steps_middle_c: Steps,
}

impl Staff {
    /// A half or whole step visual distance in the measure.
    const STEP_DY: i32 = 125;
    /// Margin X
    const MARGIN_X: i32 = BARLINE_WIDTH;
    /// Minimum number of steps in top/bottom margins
    const MARGIN_STEPS: Steps = Steps(6);
    /// Width of staff lines (looks best if it matches BARLINE_WIDTH).
    const LINE_WIDTH: i32 = BARLINE_WIDTH;

    /// Create a new staff
    pub fn new(lines: i32, steps_middle_c: Steps) -> Self {
        Staff { lines, steps_middle_c }
    }

    /// Get number of steps top margin is above middle C
    fn steps_top(&self, steps: Steps) -> Steps {
        let top = ((steps / 2) * 2).0 + 2; // round to nearest line
        let dflt = self.steps_middle_c + Self::MARGIN_STEPS;
        Steps(dflt.0.max(top))
    }

    /// Get number of steps bottom margin is above middle C
    fn steps_bottom(&self, steps: Steps) -> Steps {
        let bottom = ((steps / 2) * 2).0 - 2; // round to nearest line
        let dflt = self.steps_middle_c - self.height_steps()
            - Self::MARGIN_STEPS;
        Steps(dflt.0.min(bottom))
    }

    /// Get number of steps bottom of staff is above middle C
    fn steps_staff_bottom(&self) -> Steps {
        self.steps_middle_c - self.height_steps()
    }

    /// Get the height of the staff
    pub fn height_steps(&self) -> Steps {
        if self.lines > 0 {
            Steps(2 * (self.lines - 1))
        } else {
            Steps(0)
        }
    }

    /// Create a staff path
    pub fn path(&self, top: i32, width: i32) -> Path {
        let mut d = String::new();
        for i in 0..self.lines {
            let x = Self::MARGIN_X;
            let y = top + Staff::STEP_DY * (i * 2) - Staff::LINE_WIDTH / 2;
            let line = &format!("M{} {}h{}v{}h-{}v-{}z", x, y, width,
                Staff::LINE_WIDTH, width, Staff::LINE_WIDTH);
            d.push_str(line);
        }
        Path::new(None, d)
    }
}

pub struct MeasureElem {
    /// Staff containing the measure
    pub staff: Staff,
    /// Number of steps top margin is above middle C
    pub steps_top: Steps,
    /// Number of steps bottom margin is above middle C
    pub steps_bottom: Steps,
    /// Width of measure
    pub width: i32,
    /// SVG Elements
    pub elements: Vec<Element>,
}

impl fmt::Display for MeasureElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.elements {
            write!(f, "{}", elem)?;
        }
        Ok(())
    }
}

impl MeasureElem {
    /// Width of stems
    const STEM_WIDTH: u32 = 30;
    /// Length of stems
    const STEM_LENGTH: u32 = 7 * Staff::STEP_DY as u32;
    /// Width of note head
    const HEAD_WIDTH: i32 = 266;

    /// Create a new measure element
    pub fn new(staff: Staff, high: Steps, low: Steps) -> Self {
        let steps_top = staff.steps_top(high);
        let steps_bottom = staff.steps_bottom(low);
        let width = 0;
        let elements = vec![];
        MeasureElem { staff, steps_top, steps_bottom, width, elements }
    }

    /// Add markings to this measure.
    ///
    /// - `scof`: The score.
    /// - `curs`: Cursor of measure.
    pub fn add_markings(&mut self, scof: &Scof, curs: &mut Cursor) {
        let mut is_empty = true;

        let mut notator = Notator::new(self);
        while let Some(marking) = scof.marking(&curs) {

            is_empty = false;
            match marking {
                Marking::Note(note) => {
                    notator.notate(&note);
//                    self.width += note.duration() * BAR_WIDTH;
//                        self.add_mark(&note);
//                        self.width += fraction * BAR_WIDTH;
                }
                _ => unreachable!()
            }
            curs.right_unchecked();
        }

        // Insert whole measure rest (different from whole rest).
        // whole measure rests are always 1 measure, so can be any number of
        // beats depending on the time signature.  They look like a whole rest,
        // but are centered.
        if is_empty {
//            cala::info!("Add measure rest {}", curs.measure);
            self.add_measure_rest();

        }

            self.width += BAR_WIDTH;

        self.add_barline(self.width);
    }

    /// Add a cursor
    /// - `cursor`: Cursor position.
    pub fn add_cursor(&mut self, scof: &Scof, cursor: &Cursor) {
        let mut width = 0;
        let mut curs = cursor.first_marking();

        if cursor.is_first_bar() {
            width += 1640;
        }

        let mut is_empty = true;
        while let Some(Marking::Note(note)) = scof.marking(&curs) {
            let add = *cursor == curs;
            is_empty = false;
            self.add_cursor_rect(note.duration(), &mut width, add);
            if add { break }
            curs.right_unchecked();
        }
        if is_empty {
            let add = *cursor == curs;
            self.add_cursor_rect(Fraction::new(1, 1), &mut width, add);
        }
    }

    /// Get the Y offset of a step value
    fn offset_y(&self, steps: Steps) -> i32 {
        debug_assert!(steps.0 <= self.steps_top.0);
        ((self.steps_top - steps) * Staff::STEP_DY).0
    }

    /// Get the full height
    fn height(&self) -> u32 {
        ((self.steps_top - self.steps_bottom) * Staff::STEP_DY).0 as u32
    }

    /// Get the middle of the staff
    fn middle(&self) -> i32 {
        let steps = self.staff.steps_middle_c - self.staff.height_steps() / 2;
        self.offset_y(steps)
    }

    /// Add the cursor rectangle.
    fn add_cursor_rect(&mut self, fraction: Fraction, width: &mut i32, add: bool) {
        if add {
            let x = (Staff::MARGIN_X - BARLINE_WIDTH) + *width + BARLINE_WIDTH;
            let w = fraction * BAR_WIDTH - BARLINE_WIDTH;
            if w > 0 {
                let height = self.height();
                let width = w as u32;
                let fill = Some(CURSOR_COLOR);
                let rect = Rect::new(x, 0, width, height, None, None, fill);
                self.elements.push(Element::Rect(rect));
            }
        }
        *width += fraction * BAR_WIDTH;
    }

    /// Add a barline to staff
    fn add_barline(&mut self, x: i32) {
        let width = BARLINE_WIDTH as u32;
        let y = self.offset_y(self.staff.steps_middle_c);
        let y_bottom = self.offset_y(self.staff.steps_staff_bottom());
        let height = (y_bottom - y) as u32;
        let rect = Rect::new(x + (Staff::MARGIN_X - BARLINE_WIDTH), y, width, height, None, None, None);
        self.elements.push(Element::Rect(rect));
    }

/*    /// Add mark node for either a note or a rest
    #[deprecated]
    fn add_mark(&mut self, note: &Note) {
        match &note.pitch {
            Some(_pitch) => self.add_pitch(note),
            None => self.add_rest(Some(note)),
        }
    }*/

    /// Add elements for a note
    fn add_pitch(&mut self, dur: u16, offset: Fraction, vd: Option<scof::Steps>) {
        if let Some(steps) = vd {
            let x = (Staff::MARGIN_X - BARLINE_WIDTH) + NOTE_MARGIN + self.width + (offset * BAR_WIDTH);
            let y = self.offset_y(steps);
            let cp = GlyphId::notehead_duration(dur);
            self.add_use(cp, x, y);
            // Only draw stem if not a whole note or double whole note (breve).
            match dur {
                128 | 256 => {},
                _ => self.add_stem(x, y),
            }
            // Draw flag if 8th note or shorter.
            if let Some(flag_glyph) = GlyphId::flag_duration(dur, y > self.middle()) {
                let (ofsx, ofsy) = if y > self.middle() {
                    (Self::HEAD_WIDTH, -(Self::STEM_LENGTH as i32))
                } else {
                    (0, Self::STEM_LENGTH as i32)
                };

                self.add_use(flag_glyph, x + ofsx, y + ofsy);
            }
        }
    }

    /// Add a stem
    fn add_stem(&mut self, x: i32, y: i32) {
        if y > self.middle() {
            self.add_stem_up(x, y);
        } else {
            self.add_stem_down(x, y);
        }
    }

    /// Add a stem downwards.
    fn add_stem_down(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the staff
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(x, y, Self::STEM_WIDTH, Self::STEM_LENGTH, rx, ry,
            None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add a stem upwards.
    fn add_stem_up(&mut self, x: i32, y: i32) {
        // FIXME: stem should always reach the center line of the staff
        let rx = Some(Self::STEM_WIDTH / 2);
        let ry = Some(Self::STEM_WIDTH);
        let rect = Rect::new(x + Self::HEAD_WIDTH, y - Self::STEM_LENGTH as i32,
            Self::STEM_WIDTH, Self::STEM_LENGTH, rx, ry, None);
        self.elements.push(Element::Rect(rect));
    }

    /// Add `use` element for a whole measure rest
    fn add_measure_rest(&mut self/*, note: Option<&Note>*/) {
/*        let note = if let Some(note) = note {
            note
        } else {*/
            let x = (Staff::MARGIN_X - BARLINE_WIDTH) + (BAR_WIDTH - WHOLE_REST_WIDTH) / 2;
            let y = self.middle() - Staff::STEP_DY * 2;
            self.add_use(GlyphId::Rest1, x, y);
/*            return;
        };
        let duration = &note.duration;
        let glyph = GlyphId::rest_duration(duration);
        let x = NOTE_MARGIN + self.width;
        let mut y = self.middle();
        // Position whole rest glyph up 2 steps.
        if duration.num == duration.den {
            y -= Staff::STEP_DY * 2;
        }
        self.add_use(glyph, x, y);*/
    }

    /// Add `use` element for a rest.
    fn add_rest(&mut self, glyph: GlyphId, offset: Fraction) {
        let x = (Staff::MARGIN_X - BARLINE_WIDTH) + NOTE_MARGIN + self.width + (offset * BAR_WIDTH);
        let mut y = self.middle();
        // Position whole rest glyph up 2 steps.
        if glyph == GlyphId::Rest1 {
            y -= Staff::STEP_DY * 2;
        }
        self.add_use(glyph, x, y);
    }

    /// Add use element
    fn add_use(&mut self, glyph: GlyphId, x: i32, y: i32) {
        self.elements.push(Element::Use(Use::new(x, y, glyph.into())));
    }

    /// Add staff
    pub fn add_staff(&mut self) {
        let y = self.offset_y(self.staff.steps_middle_c);
        let path = self.staff.path(y, self.width);
        self.elements.push(Element::Path(path))
    }

    /// Add clef
    pub fn add_clef(&mut self) {
        self.add_use(GlyphId::ClefC, Staff::MARGIN_X + 150, self.middle());
        self.width += 1000;
    }

    /// Add time signature
    pub fn add_time(&mut self) {
        // width=421
        self.add_use(GlyphId::TimeSig3, Staff::MARGIN_X + self.width + 50, self.middle() - Staff::STEP_DY * 2);
        // width=470
        self.add_use(GlyphId::TimeSig4, Staff::MARGIN_X + self.width + 50 - ((470 - 421) / 2), self.middle() + Staff::STEP_DY * 2);

        self.width += 640;
    }

    /// Add clef & time signature.
    pub fn add_signature(&mut self) {
        self.add_clef();
        self.add_time();
    }
}

#[cfg(test)]
mod tests {}
