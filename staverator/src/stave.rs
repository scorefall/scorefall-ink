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
//
use scof::Steps;
use sfff::SfFontMetadata;

use crate::bar;

/// Stave lines
pub struct Stave {
    /// Number of lines on stave
    pub lines: i32,
    /// Number of steps top of stave is above middle C
    pub(crate) steps_middle_c: Steps,
    /// Y position (in steps).
    ypos: Steps,
}

impl Stave {
    /// Minimum number of steps in top/bottom margins
    const MARGIN_STEPS: Steps = Steps(6);
    /// A stave space
    pub(crate) const SPACE: i32 = bar::STAVE_SPACE;
    /// Half or whole step visual distance in the measure (half a stave space)
    pub(crate) const STEP: i32 = Self::SPACE / 2;

    /// Create a new stave
    pub fn new(lines: i32, steps_middle_c: Steps, ypos: Steps) -> Self {
        Stave {
            lines,
            steps_middle_c,
            ypos,
        }
    }

    /// Get number of steps top margin is above middle C
    pub(crate) fn steps_top(&self, steps: Steps) -> Steps {
        let top = ((steps / 2) * 2).0 + 2; // round to nearest line
        let dflt = self.steps_middle_c + Self::MARGIN_STEPS + self.ypos;
        Steps(dflt.0.max(top))
    }

    /// Get number of steps bottom margin is above middle C
    pub(crate) fn steps_bottom(&self, steps: Steps) -> Steps {
        let bottom = ((steps / 2) * 2).0 - 2; // round to nearest line
        let dflt =
            self.steps_middle_c - self.height_steps() - Self::MARGIN_STEPS
                + self.ypos;
        Steps(dflt.0.min(bottom))
    }

    /// Get number of steps bottom of stave is above middle C
    pub(crate) fn steps_stave_bottom(&self) -> Steps {
        self.steps_middle_c - self.height_steps()
    }

    /// Get the height of the stave
    pub fn height_steps(&self) -> Steps {
        if self.lines > 0 {
            Steps(2 * (self.lines - 1))
        } else {
            Steps(0)
        }
    }

    /// Create a stave path
    pub fn path(
        &self,
        meta: &SfFontMetadata,
        top: i32,
        width: i32,
        ofs: Steps,
    ) -> String {
        let ofs = (ofs * Stave::STEP).0;
        let mut d = String::new();
        for i in 0..self.lines {
            let x = 0;
            let y =
                top + Stave::SPACE * i - meta.stave_line_thickness / 2 + ofs;
            let line = &format!(
                "M{} {}h{}v{}h-{}v-{}z",
                x,
                y,
                width,
                meta.stave_line_thickness,
                width,
                meta.stave_line_thickness,
            );
            d.push_str(line);
        }
        d
    }
}
