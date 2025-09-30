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

use std::fmt;

/// SVG `path` element
pub struct Path {
    /// Element ID
    pub id: Option<String>,
    /// Path data
    pub d: String,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<path")?;
        if let Some(ref id) = self.id {
            write!(f, " id='{}'", id)?;
        }
        write!(f, " d='{}'/>", self.d)
    }
}

impl Path {
    /// Create a new SVG `path` element
    pub fn new<T: Into<String>>(id: Option<T>, d: T) -> Self {
        let id = match id {
            Some(id) => Some(id.into()),
            None => None,
        };
        let d = d.into();
        Path { id, d }
    }
}
