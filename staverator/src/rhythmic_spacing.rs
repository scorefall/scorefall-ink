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

use std::{collections::VecDeque, convert::TryInto};

use hatmil::{Html, Svg};
use scof::Steps;
use sfff::SfFontMetadata;

use crate::{
    bar::{BAR_WIDTH, BarElem},
    beaming::Beams,
    notator::Voicing,
    stave::Stave,
};

/// Time of full bar (128th notes)
const TIME_BAR: u16 = 128;

/// Engraver for a single bar of music (multiple staves)
pub struct BarEngraver<'a> {
    /// Font metadata
    meta: &'a SfFontMetadata,
    // Voicings for each stave
    stave_voicing: Vec<Vec<Voicing>>,
    // Cursors for each stave
    stave_cursor: Vec<bool>,
    // Priority Queue for the next note to render (priority: 128ths remaining)
    pq: VecDeque<(u16, usize)>,
    // Rests (stave, is_cursor)
    rests: Vec<(usize, bool)>,
    // Bar physical width
    width: f32,
    // Remaining 128th notes for all staves
    remaining: u16,
    // Cursor (x, stave)
    cursor: Option<(f32, usize)>,
    // Cursor span (x, width)
    cursor_span: Option<(i32, i32)>,
    // Keep track of which notes to beam, and which to flag.
    beams: Vec<Beams>,
}

impl<'a> BarEngraver<'a> {
    /// Create a new bar engraver
    pub(super) fn new(
        meta: &'a SfFontMetadata,
        stave_voicing: Vec<Vec<Voicing>>,
        stave_cursor: Vec<bool>,
    ) -> Self {
        // Add each stave
        let mut beams = vec![];
        let mut pq = VecDeque::new();
        for i in 0..stave_voicing.len() {
            // All 128ths remaining.
            pq.push_back((TIME_BAR, i));
            beams.push(Beams::new());
        }
        let rests = Vec::new();
        // Beginning of bar margin
        let width = Stave::SPACE as f32 / BAR_WIDTH as f32;
        let remaining = TIME_BAR;
        let cursor = None;
        let cursor_span = None;

        Self {
            meta,
            stave_voicing,
            stave_cursor,
            pq,
            rests,
            width,
            remaining,
            cursor,
            cursor_span,
            beams,
        }
    }

    /// Add to queue if time is remaining
    fn add_time(&mut self, time: u16, stave_i: usize) {
        if time > 0 {
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

    /// Pop next voicing for the given stave
    fn pop_voicing(&mut self, stave_i: usize) -> Option<Voicing> {
        let sn = &mut self.stave_voicing[stave_i];
        if sn.is_empty() {
            None
        } else {
            Some(sn.remove(0))
        }
    }

    /// Check if a stave contains the cursor
    fn is_cursor(&self, stave_i: usize) -> bool {
        self.stave_cursor[stave_i]
    }
}

impl BarElem {
    /// Engrave the bar of music.
    pub fn engrave(&mut self, mut engraver: BarEngraver<'_>) -> i32 {
        let ymargin = self.stave.height_steps() + Steps(12);
        // Empty the priority queue.
        while let Some((time, stave_i)) = engraver.pq.pop_front() {
            self.engrave_one(&mut engraver, time, stave_i);
        }
        // Beam eighth notes and shorter.
        while let Some(beam) = engraver.beams.pop() {
            self.add_flags_and_beams(engraver.meta, beam);
        }
        // Add the rest of the width.
        engraver.width += get_spacing(engraver.remaining) / 7.0;
        // End of bar margin
        engraver.width += Stave::SPACE as f32 / BAR_WIDTH as f32;
        // Draw measure rests
        for (rest_stave, rest_ic) in &engraver.rests {
            let steps = ymargin * (*rest_stave as i32);
            self.add_measure_rest(engraver.width, steps);
            if *rest_ic {
                engraver.cursor_span = Some((
                    engraver.meta.barline_thickness,            // X
                    (BAR_WIDTH as f32 * engraver.width) as i32, // W
                ));
            }
        }
        // Cursor at end of bar.
        if let Some((x, _stave_j)) = engraver.cursor {
            engraver.cursor = None;
            let e = if x == 0.0 {
                0
            } else {
                -engraver.meta.barline_thickness
            };
            let x = (BAR_WIDTH as f32 * x) as i32;
            engraver.cursor_span = Some((
                x + e, // X
                engraver.meta.barline_thickness
                    + (BAR_WIDTH as f32 * engraver.width) as i32
                    - x
                    - e, // W
            ));
        }
        // Calculate physical bar width.
        let bar_width =
            ((BAR_WIDTH as f32 * engraver.width) as i32).max(BAR_WIDTH);
        // Draw barlines
        for i in 0..engraver.stave_voicing.len().try_into().unwrap() {
            let y = self.offset_y(self.stave.steps_middle_c);
            let d = self.stave.path(engraver.meta, y, bar_width, ymargin * i);
            let mut html = Html::new();
            Svg::new(&mut html).path().d(d).end();
            self.elements.push(html);
            self.add_barline(engraver.meta, bar_width, ymargin * i);
        }

        if let Some((x, w)) = engraver.cursor_span {
            let mut html = Html::new();
            Svg::new(&mut html)
                .rect()
                .id("cursor")
                .x(x)
                .y(0)
                .width(w)
                .height(self.height())
                .fill("#FF9AF0")
                .end();
            self.elements.insert(0, html);
        }

        // Return calculated physical bar width.
        bar_width
    }

    /// Engrave one marking
    fn engrave_one(
        &mut self,
        engraver: &mut BarEngraver<'_>,
        mut time: u16,
        stave_i: usize,
    ) {
        let Some(voicing) = engraver.pop_voicing(stave_i) else {
            engraver.rests.push((stave_i, engraver.is_cursor(stave_i)));
            return;
        };
        // Increment width
        if time < engraver.remaining {
            engraver.width += get_spacing(engraver.remaining - time) / 7.0;
            engraver.remaining = time;
        }
        // Render cursor
        if voicing.is_cursor {
            if engraver.cursor.is_none() {
                if time == TIME_BAR {
                    // If first thing, cursor takes up margin.
                    engraver.cursor = Some((0.0, stave_i));
                } else {
                    engraver.cursor = Some((engraver.width, stave_i));
                }
            }
        } else if let Some((x, stave_j)) = engraver.cursor {
            if stave_i == stave_j {
                engraver.cursor = None;
                let e = if x == 0.0 {
                    0
                } else {
                    -engraver.meta.barline_thickness
                };
                let f = if x == 0.0 {
                    -engraver.meta.barline_thickness
                } else {
                    0
                };
                let x = if x == 0.0 {
                    engraver.meta.barline_thickness
                } else {
                    0
                } + (BAR_WIDTH as f32 * x) as i32;
                engraver.cursor_span = Some((
                    x + e,                                              // X
                    (BAR_WIDTH as f32 * engraver.width) as i32 - x + f, // W
                ));
            }
        }
        let ymargin = self.stave.height_steps() + Steps(12);
        // Render pitch or rest.
        if voicing.pitches.is_empty() {
            // Add rest
            self.add_rest(
                crate::glyph::rest_duration(voicing.dur),
                engraver.width,
                ymargin * stave_i as i32,
            );
            // Advance beaming
            engraver.beams[stave_i].advance(voicing.dur, engraver.width, None);
        } else {
            // Offset Y, so that the note appears on the correct stave.
            let y_offset = ymargin * stave_i as i32;
            // Add chord
            for pitch in &voicing.pitches {
                let y = self.y_from_steps(pitch.visual_distance(), y_offset);

                self.add_pitch(
                    engraver.meta,
                    voicing.dur,
                    engraver.width,
                    pitch.visual_distance(),
                    y,
                );
            }
            // Advance beaming (using closest note to the beam)
            engraver.beams[stave_i].advance(
                voicing.dur,
                engraver.width,
                Some((voicing.pitches.clone(), y_offset)),
            );
        }
        time -= voicing.dur;
        // Add back to queue if time is remaining.
        engraver.add_time(time, stave_i);
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
