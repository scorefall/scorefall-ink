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

/// 
pub struct BarEngraver {
    
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
        512 => 10.0, // Quadruple Whole
        _ => panic!("Bug in Notator, no glyph for ({})", duration),
    }
}
