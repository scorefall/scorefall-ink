// ScoreFall Ink - Music Composition Software
//
// Copyright (C) 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
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

#![allow(clippy::blacklisted_name)] // bar is a useful musical term

mod bar;
mod beaming;
mod glyph;
mod notator;
mod notehead;
mod rhythmic_spacing;
mod stave;

use devout::{Tag, log};
use hatmil::{Html, Svg};
use scof::{Cursor, Scof, Steps};
use sfff::Glyph;
pub use sfff::SfFontMetadata;

pub use bar::{BarElem, STAVE_SPACE};
pub use stave::Stave;

/// Get Modern font data as SVG defs.
pub fn modern() -> (sfff::SfFontMetadata, String) {
    let data: &[u8] = include_bytes!("../modern.sfff");
    let data = std::io::Cursor::new(data);
    let (meta, glyphs) = sfff::SfFontMetadata::from_buf_reader(data).unwrap();
    let glyphs = sfff::generate_defs(&glyphs);

    (meta, glyphs)
}
