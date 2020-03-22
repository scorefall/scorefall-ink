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

//! Render a bar for all parts.  This not only handles space between notes, but
//! also calculates the required width of the bar.

use std::collections::VecDeque;
use std::convert::TryInto;

use crate::{BarElem, Element, GlyphId, Notator, Stave, Beams};
use scof::Steps;

/// Engraver for a single bar of music (multiple staves)
pub struct BarEngraver<'a, 'b, 'c> {
    // Priority Queue for the next note to render (priority: 128ths remaining)
    pq: VecDeque<(u16, usize)>,
    //
    notators: &'a mut [Notator<'c>],
    //
    bar: &'b mut BarElem,
    // Bar physical width
    width: f32,
    // Remaining 128th notes for all staves
    all: u16,
    //
    cursor: Option<(f32, usize)>,
    // Keep track of which notes to beam, and which to flag.
    beams: Vec<Beams>,
}

impl<'a, 'b, 'c> BarEngraver<'a, 'b, 'c> {
    /// Create a new bar engraver from .
    pub(super) fn new(
        bar: &'b mut BarElem,
        notators: &'a mut [Notator<'c>],
    ) -> Self {
        // Add each stave
        let mut beams = vec![];
        let mut pq = VecDeque::new();
        for i in 0..notators.len() {
            // 128 128ths remaining.
            pq.push_back((128, i));
            beams.push(Beams::new());
        }
        // Beginning of bar margin
        let width = Stave::SPACE as f32 / super::BAR_WIDTH as f32;
        let all = 128;
        let cursor = None;

        Self {
            pq,
            notators,
            bar,
            width,
            all,
            cursor,
            beams,
        }
    }

    /// Engrave the bar of music.
    pub fn engrave(&mut self) -> (i32, Option<(i32, i32, i32, i32)>) {
        let ymargin = self.bar.stave.height_steps() + Steps(12);
        let mut cursor_rect = None;
        let mut rests = vec![];
        self.cursor = None;
        // Empty the priority queue.
        while let Some((mut time, stave_i)) = self.pq.pop_front() {
            let (pitches, dur, ic) =
                if let Some(a) = self.notators[stave_i].next() {
                    a
                } else {
                    rests.push((stave_i, self.notators[stave_i].is_cursor()));
                    continue;
                };
            // Increment width
            if time < self.all {
                self.width += get_spacing(self.all - time) / 7.0;
                self.all = time;
            }
            // Render cursor
            if ic {
                if self.cursor.is_none() {
                    if time == 128 {
                        // If first thing, cursor takes up margin.
                        self.cursor = Some((0.0, stave_i));
                    } else {
                        self.cursor = Some((self.width, stave_i));
                    }
                }
            } else if let Some((x, stave_j)) = self.cursor {
                if stave_i == stave_j {
                    self.cursor = None;
                    let e = if x == 0.0 { 0 } else { -Stave::STEP };
                    let f = if x == 0.0 { -Stave::STEP } else { 0 };
                    let x =
                        Stave::MARGIN_X + (super::BAR_WIDTH as f32 * x) as i32;
                    cursor_rect = Some((
                        x + e, // X
                        0i32,  // Y
                        (super::BAR_WIDTH as f32 * self.width) as i32 - x
                            + f
                            + Stave::MARGIN_X, // W
                        self.bar.height(),
                    ));
                }
            }
            // Render pitch or rest.
            if pitches.is_empty() {
                // Add rest
                self.bar.add_rest(
                    GlyphId::rest_duration(dur),
                    self.width,
                    ymargin * stave_i as i32,
                );
                // Advance beaming
                self.beams[stave_i].advance(dur, self.width, None);
            } else {
                // Offset Y, so that the note appears on the correct stave.
                let y_offset = ymargin * stave_i as i32;
                // Add chord
                for pitch in &pitches {
                    let y = self.bar.y_from_steps(pitch.visual_distance(),
                        y_offset);

                    self.bar.add_pitch(
                        dur,
                        self.width,
                        pitch.visual_distance(),
                        y,
                    );
                }
                // Advance beaming (using closest note to the beam)
                self.beams[stave_i].advance(dur, self.width, Some((pitches.clone(), y_offset)));
            }
            // Add back to queue if time is remaining.
            time -= dur;
            if time != 0 {
                // Insert at correct priority level.
                let mut index = self.pq.len();
                'p: loop {
                    if index == 0 {
                        self.pq.push_front((time, stave_i));
                        break 'p;
                    }
                    index -= 1;
                    if self.pq[index].0 > time {
                        self.pq.push_back((time, stave_i));
                        break 'p;
                    }
                }
            }
        }
        // Beam eighth notes and shorter.
        while let Some(beam) = self.beams.pop() {
            self.bar.add_flags_and_beams(beam);
        }
        // Add the rest of the width.
        self.width += get_spacing(self.all) / 7.0;
        // End of bar margin
        self.width += Stave::SPACE as f32 / super::BAR_WIDTH as f32;
        // Draw measure rests
        for (rest_stave, rest_ic) in rests {
            self.bar
                .add_measure_rest(self.width, ymargin * rest_stave as i32);
            if rest_ic {
                cursor_rect = Some((
                    crate::Stave::MARGIN_X, // X
                    0i32,                   // Y
                    (super::BAR_WIDTH as f32 * self.width) as i32, // W
                    self.bar.height(),
                ));
            }
        }
        // Cursor at end of bar.
        if let Some((x, _stave_j)) = self.cursor {
            self.cursor = None;
            let e = if x == 0.0 { 0 } else { -Stave::STEP };
            let x = (super::BAR_WIDTH as f32 * x) as i32;
            cursor_rect = Some((
                crate::Stave::MARGIN_X + x + e,                        // X
                0i32,                                                  // Y
                (super::BAR_WIDTH as f32 * self.width) as i32 - x - e, // W
                self.bar.height(),
            ));
        }
        // Calculate physical bar width.
        let bar_width = ((super::BAR_WIDTH as f32 * self.width) as i32)
            .max(super::BAR_WIDTH);
        // Draw barlines
        for i in 0..self.notators.len().try_into().unwrap() {
            let y = self.bar.offset_y(self.bar.stave.steps_middle_c);
            let path = self.bar.stave.path(y, bar_width, ymargin * i);
            self.bar.elements.push(Element::Path(path));
            self.bar.add_barline(bar_width, ymargin * i);
        }
        // Return calculated physical bar width.
        (bar_width, cursor_rect)
    }
}

/// Linear interpolation
fn lerp(a: f32, b: f32, amount: f32) -> f32 {
    a * amount + b * (1.0 - amount)
}

/// Clamp a between min and max
fn clamp(a: f32, min: f32, max: f32) -> f32 {
    (a - min) / (max - min)
}

/// Get the fraction of the spacing of a whole note that this note needs based
/// on duration (in 128th notes).
fn get_spacing(duration: u16) -> f32 {
    let dur = duration as f32;
    match duration {
        1..=7 => lerp(1.8, 2.0, clamp(dur, 1.0, 8.0)), // 128th-16th
        8..=15 => lerp(2.0, 2.5, clamp(dur, 8.0, 16.0)), // Sixteenth
        16..=23 => lerp(2.5, 3.0, clamp(dur, 16.0, 24.0)), // Eighth
        24..=31 => lerp(3.0, 3.5, clamp(dur, 24.0, 32.0)), // Dot'd Eighth
        32..=47 => lerp(3.5, 4.0, clamp(dur, 32.0, 48.0)), // Quarter
        48..=63 => lerp(4.0, 5.0, clamp(dur, 48.0, 64.0)), // Dot'd Quarter
        64..=95 => lerp(5.0, 6.0, clamp(dur, 64.0, 96.0)), // Half
        96..=127 => lerp(6.0, 7.0, clamp(dur, 96.0, 128.0)), // Dotted Half
        128..=255 => lerp(7.0, 8.0, clamp(dur, 128.0, 256.0)), // Whole
        256..=383 => lerp(8.0, 9.0, clamp(dur, 256.0, 384.0)), // Dot'd Whole
        384..=511 => lerp(9.0, 10.0, clamp(dur, 384.0, 512.0)), // Breve
        512 => 10.0,                                      // Longa
        _ => panic!("Bug in Notator, no glyph for ({})", duration),
    }
}
