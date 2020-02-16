// ScoreFall Studio - Music Composition Software
//
// Copyright (C) 2019 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
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

use std::fmt;

/// SVG `rect` element
pub struct Rect {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Width in user units
    pub width: u32,
    /// Height in user units
    pub height: u32,
    /// Rounded corner X-axis radius
    pub rx: Option<u32>,
    /// Rounded corner Y-axis radius
    pub ry: Option<u32>,
    /// Fill color
    pub fill: Option<String>,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<rect x='{}' y='{}' width='{}' height='{}'",
            self.x, self.y, self.width, self.height)?;
        if let Some(ref rx) = self.rx {
            write!(f, " rx='{}'", rx)?;
        }
        if let Some(ref ry) = self.ry {
            write!(f, " ry='{}'", ry)?;
        }
        if let Some(ref fill) = self.fill {
            write!(f, " fill='{}'", fill)?;
        }
        write!(f, "/>")
    }
}

impl Rect {
    /// Create a new SVG `rect` element
    pub fn new(x: i32, y: i32, width: u32, height: u32, rx: Option<u32>,
        ry: Option<u32>, fill: Option<u32>) -> Self
    {
        let fill = match fill {
            Some(f) => Some(format!("#{:x}", f)),
            None => None,
        };
        Rect { x, y, width, height, rx, ry, fill }
    }
}

/// SVG `use` element
pub struct Use {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Element ID
    pub id: u32,
}

impl fmt::Display for Use {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<use x='{}' y='{}' xlink:href='#{:x}'/>", self.x,
            self.y, self.id)
    }
}

impl Use {
    /// Create a new SVG `use` element
    pub fn new(x: i32, y: i32, id: u32) -> Self {
        Use { x, y, id }
    }
}

/// SVG group `g` element
pub struct Group {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Elements within group
    pub elements: Vec<Element>,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<g")?;
        if self.x != 0 && self.y != 0 {
            write!(f, " transform='translate({} {})'>", self.x, self.y)?;
        } else {
            write!(f, ">")?;
        }
        for elem in &self.elements {
            write!(f, "{}", elem)?;
        }
        write!(f, "</g>")
    }
}

impl Group {
    /// Create a new SVG group `g` element
    pub fn new(x: i32, y: i32) -> Self {
        let elements = vec![];
        Group { x, y, elements }
    }
    /// Push an element into the group
    pub fn push(&mut self, elem: Element) {
        self.elements.push(elem);
    }
}

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

/// SVG element
pub enum Element {
    /// Group `g`
    Group(Group),
    /// `rect`
    Rect(Rect),
    /// `use`
    Use(Use),
    /// `path`
    Path(Path),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Group(g) => g.fmt(f),
            Element::Rect(r) => r.fmt(f),
            Element::Use(u) => u.fmt(f),
            Element::Path(p) => p.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::glyph::GlyphId;

    #[test]
    fn rect() {
        assert_eq!(Rect::new(10, 12, 25, 20, None, None, None).to_string(),
        "<rect x='10' y='12' width='25' height='20'/>");
    }

    #[test]
    fn glyph() {
        assert_eq!(Use::new(37, 21, GlyphId::StemHarpStringNoise.into()).to_string(),
        "<use x='37' y='21' xlink:href='#e21f'/>");
    }

    #[test]
    fn group() {
        let mut group = Group::new(0, 0);
        group.push(Element::Use(Use::new(2, 3, GlyphId::NoteheadWhole.into())));
        assert_eq!(group.to_string(),
        "<g><use x='2' y='3' xlink:href='#e0a2'/></g>");
    }
}
