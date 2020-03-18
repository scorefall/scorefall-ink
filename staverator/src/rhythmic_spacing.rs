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

/* ************************************************************************** */

//! Render a bar for all parts.  This not only handles space between notes, but
//! also calculates the required width of the bar.

use std::collections::VecDeque;
use std::convert::TryInto;

use crate::{BarElem, Element, GlyphId, Notator};
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
    // Remaining 128th notes for each stave
    rem: Vec<u16>,
    // Remaining 128th notes for all staves
    all: u16,
    //
    cursor: Option<(f32, usize)>,
}

impl<'a, 'b, 'c> BarEngraver<'a, 'b, 'c> {
    /// Create a new bar engraver from .
    pub(super) fn new(
        bar: &'b mut BarElem,
        notators: &'a mut [Notator<'c>],
    ) -> Self {
        // Add each stave
        let mut pq = VecDeque::new();
        for i in 0..notators.len() {
            // 128 128ths remaining.
            pq.push_back((128, i));
        }
        let width = 0.0;
        let rem = vec![128; notators.len()];
        let all = 128;
        let cursor = None;

        Self {
            pq,
            notators,
            bar,
            width,
            rem,
            all,
            cursor,
        }
    }

    /// Engrave the bar of music.
    pub fn engrave(&mut self) -> (i32, Option<(i32, i32, i32, i32)>) {
        let ymargin = self.bar.stave.height_steps() + Steps(12);
        let mut cursor_rect = None;
        let mut rests = vec![];
        self.cursor = None;
        // Empty the priority queue.
        while let Some((time, stave_i)) = self.pq.pop_front() {
            let (pitches, dur, ic) =
                if let Some(a) = self.notators[stave_i].next() {
                    a
                } else {
                    rests.push((stave_i, self.notators[stave_i].is_cursor()));
                    continue;
                };
            // Increment width
            if self.rem[stave_i] < self.all {
                self.width += get_spacing(self.all - self.rem[stave_i]) / 7.0;
                self.all = self.rem[stave_i];
            }
            self.rem[stave_i] -= dur;
            // Render cursor
            if ic {
                if self.cursor.is_none() {
                    self.cursor = Some((self.width, stave_i));
                }
            } else if let Some((x, stave_j)) = self.cursor {
                if stave_i == stave_j {
                    self.cursor = None;
                    let x = crate::Stave::MARGIN_X
                        + (super::BAR_WIDTH as f32 * x) as i32;
                    cursor_rect = Some((
                        x,                                                 // X
                        0i32,                                              // Y
                        (super::BAR_WIDTH as f32 * self.width) as i32 - x, // W
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
            } else {
                for pitch in pitches {
                    self.bar.add_pitch(
                        dur,
                        self.width,
                        pitch.visual_distance(),
                        ymargin * stave_i as i32,
                    );
                }
            }
            // Add back to queue if time is remaining.
            let time = time - dur;
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
        // Add the rest of the width.
        self.width += get_spacing(self.all) / 7.0;
        // Draw measure rests
        for (rest_stave, rest_ic) in rests {
            self.bar
                .add_measure_rest(self.width, ymargin * rest_stave as i32);
            if rest_ic {
                cursor_rect = Some((
                    crate::Stave::MARGIN_X, // X
                    0i32,                   // Y
                    (super::BAR_WIDTH as f32 * self.width) as i32
                        - crate::Stave::MARGIN_X, // W
                    self.bar.height(),
                ));
            }
        }
        // Cursor at end of bar.
        if let Some((x, stave_j)) = self.cursor {
            self.cursor = None;
            let x =
                crate::Stave::MARGIN_X + (super::BAR_WIDTH as f32 * x) as i32;
            cursor_rect = Some((
                x,                                                 // X
                0i32,                                              // Y
                (super::BAR_WIDTH as f32 * self.width) as i32 - x, // W
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
        1..=7 => lerp(1.75, 2.0, clamp(dur, 1.0, 8.0)), // 128th-Sixteenth
        8..=15 => lerp(2.0, 2.5, clamp(dur, 8.0, 16.0)), // Sixteenth
        16..=23 => lerp(2.5, 3.0, clamp(dur, 16.0, 24.0)), // Eighth
        24..=31 => lerp(3.0, 3.5, clamp(dur, 24.0, 32.0)), // Dotted Eighth
        32..=47 => lerp(3.5, 4.0, clamp(dur, 32.0, 48.0)), // Quarter
        48..=63 => lerp(4.0, 5.0, clamp(dur, 48.0, 64.0)), // Dotted Quarter
        64..=95 => lerp(5.0, 6.0, clamp(dur, 64.0, 96.0)), // Half
        96..=127 => lerp(6.0, 7.0, clamp(dur, 96.0, 128.0)), // Dotted Half
        128..=255 => lerp(7.0, 8.0, clamp(dur, 128.0, 256.0)), // Whole
        256..=383 => lerp(8.0, 9.0, clamp(dur, 256.0, 384.0)), // Dotted Whole
        384..=511 => lerp(9.0, 10.0, clamp(dur, 384.0, 512.0)), // Double Whole
        512 => 10.0,                                    // Quadruple Whole
        _ => panic!("Bug in Notator, no glyph for ({})", duration),
    }
}
