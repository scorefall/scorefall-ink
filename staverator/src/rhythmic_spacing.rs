// ScoreFall Ink - Music Composition Software
//
// Copyright (C) 2019-2020 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
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

/* ************************************************************************** */

//! Render a bar for all parts.  This not only handles space between notes, but
//! also calculates the required width of the bar.

use std::collections::VecDeque;
use std::convert::TryInto;

use crate::{BarElem, Beams, Notator, Stave, BAR_WIDTH};
use hatmil::{Html, Svg};
use scof::Steps;
use sfff::SfFontMetadata;

/// Engraver for a single bar of music (multiple staves)
pub struct BarEngraver<'a, 'b, 'c> {
    /// Font metadata
    meta: &'b SfFontMetadata,
    // Priority Queue for the next note to render (priority: 128ths remaining)
    pq: VecDeque<(u16, usize)>,
    // Slice of notators for each stave
    notators: &'a mut [Notator<'c>],
    // Rests (stave, is_cursor)
    rests: Vec<(usize, bool)>,
    // Bar element to engrave
    bar: &'b mut BarElem,
    // Bar physical width
    width: f32,
    // Remaining 128th notes for all staves
    all: u16,
    // Cursor (x, stave)
    cursor: Option<(f32, usize)>,
    // Cursor rectangle
    cursor_rect: Option<(i32, i32, i32, i32)>,
    // Keep track of which notes to beam, and which to flag.
    beams: Vec<Beams>,
}

impl<'a, 'b, 'c> BarEngraver<'a, 'b, 'c> {
    /// Create a new bar engraver from .
    pub(super) fn new(
        bar: &'b mut BarElem,
        meta: &'b SfFontMetadata,
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
        let rests = Vec::new();
        // Beginning of bar margin
        let width = Stave::SPACE as f32 / BAR_WIDTH as f32;
        let all = 128;
        let cursor = None;
        let cursor_rect = None;

        Self {
            meta,
            pq,
            notators,
            rests,
            bar,
            width,
            all,
            cursor,
            cursor_rect,
            beams,
        }
    }

    /// Engrave the bar of music.
    pub fn engrave(&mut self) -> i32 {
        let ymargin = self.bar.stave.height_steps() + Steps(12);
        self.cursor = None;
        // Empty the priority queue.
        while let Some((time, stave_i)) = self.pq.pop_front() {
            self.engrave_one(time, stave_i);
        }
        // Beam eighth notes and shorter.
        while let Some(beam) = self.beams.pop() {
            self.bar.add_flags_and_beams(self.meta, beam);
        }
        // Add the rest of the width.
        self.width += get_spacing(self.all) / 7.0;
        // End of bar margin
        self.width += Stave::SPACE as f32 / BAR_WIDTH as f32;
        // Draw measure rests
        for (rest_stave, rest_ic) in &self.rests {
            let steps = ymargin * (*rest_stave as i32);
            self.bar.add_measure_rest(self.width, steps);
            if *rest_ic {
                self.cursor_rect = Some((
                    self.meta.barline_thickness,            // X
                    0i32,                                   // Y
                    (BAR_WIDTH as f32 * self.width) as i32, // W
                    self.bar.height(),
                ));
            }
        }
        // Cursor at end of bar.
        if let Some((x, _stave_j)) = self.cursor {
            self.cursor = None;
            let e = if x == 0.0 { 0 } else { -self.meta.barline_thickness };
            let x = (BAR_WIDTH as f32 * x) as i32;
            self.cursor_rect = Some((
                x + e, // X
                0i32,  // Y
                self.meta.barline_thickness + (BAR_WIDTH as f32 * self.width) as i32
                    - x
                    - e, // W
                self.bar.height(),
            ));
        }
        // Calculate physical bar width.
        let bar_width = ((BAR_WIDTH as f32 * self.width) as i32).max(BAR_WIDTH);
        // Draw barlines
        for i in 0..self.notators.len().try_into().unwrap() {
            let y = self.bar.offset_y(self.bar.stave.steps_middle_c);
            let d = self.bar.stave.path(self.meta, y, bar_width, ymargin * i);
            let mut html = Html::new();
            Svg::new(&mut html).path().d(d).end();
            self.bar.elements.push(html);
            self.bar.add_barline(self.meta, bar_width, ymargin * i);
        }

        if let Some((x, y, w, h)) = self.cursor_rect {
            let mut html = Html::new();
            Svg::new(&mut html)
                .rect()
                .id("cursor")
                .x(x)
                .y(y)
                .width(w)
                .height(h)
                .fill("#FF9AF0")
                .end();
            self.bar.elements.insert(0, html);
        }

        // Return calculated physical bar width.
        bar_width
    }

    /// Engrave one marking
    fn engrave_one(&mut self, mut time: u16, stave_i: usize) {
        let ymargin = self.bar.stave.height_steps() + Steps(12);
        let (pitches, dur, ic) =
            if let Some(a) = self.notators[stave_i].next() {
                a
            } else {
                self.rests.push((stave_i, self.notators[stave_i].is_cursor()));
                return;
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
                let e = if x == 0.0 { 0 } else { -self.meta.barline_thickness };
                let f = if x == 0.0 { -self.meta.barline_thickness } else { 0 };
                let x = if x == 0.0 { self.meta.barline_thickness } else { 0 }
                    + (BAR_WIDTH as f32 * x) as i32;
                self.cursor_rect = Some((
                    x + e,                                          // X
                    0i32,                                           // Y
                    (BAR_WIDTH as f32 * self.width) as i32 - x + f, // W
                    self.bar.height(),
                ));
            }
        }
        // Render pitch or rest.
        if pitches.is_empty() {
            // Add rest
            self.bar.add_rest(
                crate::glyph::rest_duration(dur),
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
                let y = self
                    .bar
                    .y_from_steps(pitch.visual_distance(), y_offset);

                self.bar.add_pitch(
                    self.meta,
                    dur,
                    self.width,
                    pitch.visual_distance(),
                    y,
                );
            }
            // Advance beaming (using closest note to the beam)
            self.beams[stave_i].advance(
                dur,
                self.width,
                Some((pitches.clone(), y_offset)),
            );
        }
        // Add back to queue if time is remaining.
        time -= dur;
        if time != 0 {
            // Insert at correct priority level.
            let mut index = self.pq.len();
            loop {
                if index == 0 {
                    self.pq.push_front((time, stave_i));
                    return;
                }
                index -= 1;
                if self.pq[index].0 > time {
                    self.pq.push_back((time, stave_i));
                    return;
                }
            }
        }
    }
}

/// Map a value from range `in_min..=in_max` to range `out_min..=out_max`.
fn map_range<const IN_MIN: u16, const IN_MAX: u16>(
    value: u16,
    out_min: f32,
    out_max: f32,
) -> f32 {
    let slope = (out_max - out_min) / f32::from(IN_MAX - IN_MIN);

    f32::from(value - IN_MIN).mul_add(slope, out_min)
}

/// Get the fraction of the spacing of a whole note that this note needs based
/// on duration (in 128th notes).
fn get_spacing(duration: u16) -> f32 {
    match duration {
        // 128th-16th
        1..=7 => map_range::<1, 8>(duration, 1.8, 2.0),
        // Sixteenth
        8..=15 => map_range::<8, 16>(duration, 2.0, 2.5),
        // Eighth
        16..=23 => map_range::<16, 24>(duration, 2.5, 3.0),
        // Dot'd Eighth
        24..=31 => map_range::<24, 32>(duration, 3.0, 3.5),
        // Quarter
        32..=47 => map_range::<32, 48>(duration, 3.5, 4.0),
        // Dot'd Quarter
        48..=63 => map_range::<48, 64>(duration, 4.0, 5.0),
        // Half
        64..=95 => map_range::<64, 96>(duration, 5.0, 6.0),
        // Dotted Half
        96..=127 => map_range::<96, 128>(duration, 6.0, 7.0),
        // Whole
        128..=255 => map_range::<128, 256>(duration, 7.0, 8.0),
        // Dot'd Whole
        256..=383 => map_range::<256, 384>(duration, 8.0, 9.0),
        // Breve
        384..=511 => map_range::<384, 512>(duration, 9.0, 10.0),
        // Longa
        512 => 10.0,
        _ => panic!("Bug in Notator, no glyph for ({})", duration),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range_works() {
        assert_eq!(2.0, map_range::<3, 5>(3, 2.0, 3.0));
        assert_eq!(2.5, map_range::<3, 5>(4, 2.0, 3.0));
        assert_eq!(3.0, map_range::<3, 5>(5, 2.0, 3.0));
    }
}
