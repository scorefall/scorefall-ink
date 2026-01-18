// ScoreFall Ink - Music Composition Software
//
// Copyright © 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
// Copyright © 2019-2026 Doug P. Lau
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

use std::task::Context;

use devout::{Tag, log};
use hatmil::{
    Page,
    svg::{Defs, G},
};
use human::{Input, Key};
use pasts::prelude::{Notify, Pin, Poll};
use scof::{Fraction, Pitch, Steps};
use scorefall_ink::Program;
use staverator::{BarElem, STAVE_SPACE, SfFontMetadata, Stave};

use crate::screen::Screen;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const ZOOM_LEVEL: f32 = 1.0;
// Stave spaces for window height.
const WINDOW_HEIGHT_SS: i32 = 64;
const SCALEDOWN: f32 = (STAVE_SPACE * WINDOW_HEIGHT_SS) as f32 / ZOOM_LEVEL;

const RENDER: Tag = Tag::new("Render");
const GUI: Tag = Tag::new("Gui");

/// Adapter for using a repeating future as a pasts notify
pub struct Adapter<F>(F)
where
    F: Future + Unpin;

impl<F> Notify for Adapter<F>
where
    F: Future + Unpin,
{
    type Event = F::Output;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.0).poll(cx)
    }
}

type InputNotifier = Adapter<Pin<Box<dyn Future<Output = Input>>>>;
type ResizeNotifier = Adapter<Pin<Box<dyn Future<Output = (u32, u32)>>>>;

/// Program state
pub struct State {
    /// The web front-end.
    screen: Screen,
    /// Front-end agnostic program state
    program: Program,
    /// Font metadata
    meta: SfFontMetadata,
    /// Window width in Stave Spaces.
    width: f32,
    /// Event sources
    pub input: InputNotifier,
    /// Resize sources
    pub resize: ResizeNotifier,
}

impl State {
    /// Create new program state
    pub fn new() -> State {
        let mut screen = Screen::new().expect("Failed to create screen");
        let input: Pin<Box<dyn Future<Output = Input>>> =
            Box::pin(Input::listener());
        let resize: Pin<Box<dyn Future<Output = (u32, u32)>>> =
            Box::pin(screen.resize());

        State {
            screen,
            program: Program::new(),
            meta: staverator::modern(),
            width: 0.0,
            input: Adapter(input),
            resize: Adapter(resize),
        }
    }

    /// Render the score
    pub fn render_score(&mut self) {
        let mut page = Page::new();
        let mut defs = page.frag::<Defs>();
        // render each glyph as a path in defs section
        for (id, path) in self.meta.glyph_paths.iter().enumerate() {
            defs.path().id(format!("{id:x}")).d(path).close();
        }
        let mut score = String::from(page);
        let mut page = Page::new();
        page.frag::<G>().id("page");
        score.push_str(&String::from(page));
        self.screen.set_svg(&score);
        self.resize(self.screen.size()).unwrap();
        self.render_page();
    }

    /// Handle input event
    pub fn event_input(&mut self, input: Input) -> Poll {
        match input {
            Input::Key(mods, key, true)
                if mods.ctrl() && matches!(key, Key::H | Key::Left) =>
            {
                // TODO: Halve duration
            }
            Input::Key(mods, key, true)
                if mods.ctrl() && matches!(key, Key::J | Key::Down) =>
            {
                self.program.down_half_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.ctrl() && matches!(key, Key::K | Key::Up) =>
            {
                self.program.up_half_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.ctrl() && matches!(key, Key::L | Key::Right) =>
            {
                // TODO: Double duration
            }
            Input::Key(mods, key, true)
                if mods.alt() && matches!(key, Key::H | Key::Left) =>
            {
                // TODO: Move selection to the left
            }
            Input::Key(mods, key, true)
                if mods.alt() && matches!(key, Key::J | Key::Down) =>
            {
                self.program.down_quarter_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.alt() && matches!(key, Key::K | Key::Up) =>
            {
                self.program.up_quarter_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.alt() && matches!(key, Key::L | Key::Right) =>
            {
                // TODO: Move selection to the right
            }
            Input::Key(mods, key, true)
                if mods.shift() && matches!(key, Key::H | Key::Left) =>
            {
                // TODO: Select left
            }
            Input::Key(mods, key, true)
                if mods.shift() && matches!(key, Key::J | Key::Down) =>
            {
                // TODO: Select down
            }
            Input::Key(mods, key, true)
                if mods.shift() && matches!(key, Key::K | Key::Up) =>
            {
                // TODO: Select up
            }
            Input::Key(mods, key, true)
                if mods.shift() && matches!(key, Key::L | Key::Right) =>
            {
                // TODO: Select right
            }
            Input::Key(mods, key, true)
                if mods.none() && matches!(key, Key::H | Key::Left) =>
            {
                self.program.left();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.none() && matches!(key, Key::J | Key::Down) =>
            {
                self.program.down_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.none() && matches!(key, Key::K | Key::Up) =>
            {
                self.program.up_step();
                self.render_page();
            }
            Input::Key(mods, key, true)
                if mods.none() && matches!(key, Key::L | Key::Right) =>
            {
                self.program.right();
                self.render_page();
            }
            Input::Key(mods, Key::One, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 64));
                self.render_page();
            }
            Input::Key(mods, Key::Two, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 32));
                self.render_page();
            }
            Input::Key(mods, Key::Three, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 16));
                self.render_page();
            }
            Input::Key(mods, Key::Four, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 8));
                self.render_page();
            }
            Input::Key(mods, Key::Five, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 4));
                self.render_page();
            }
            Input::Key(mods, Key::Six, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 2));
                self.render_page();
            }
            Input::Key(mods, Key::Seven, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 1));
                self.render_page();
            }
            Input::Key(mods, Key::Eight, true) if mods.none() => {
                self.program.set_dur(Fraction::new(2, 1));
                self.render_page();
            }
            Input::Key(mods, Key::Nine, true) if mods.none() => {
                self.program.set_dur(Fraction::new(4, 1));
                self.render_page();
            }
            Input::Key(mods, Key::Period, true) if mods.none() => {
                self.program.dotted();
                self.render_page();
            }
            _ => { /* ignore all other input */ }
        }

        Poll::Pending
    }

    /// Handle resize event
    pub fn event_resize(&mut self, size: (u32, u32)) -> Poll {
        self.resize(size).ok();
        Poll::Pending
    }

    /// Resize the SVG
    pub fn resize(&mut self, size: (u32, u32)) -> Result<()> {
        log!(GUI, "Resize {:?}", size);
        let ratio: f32 = size.0 as f32 / size.1 as f32;
        let width = SCALEDOWN * ratio;
        let height = SCALEDOWN;
        let viewbox = format!("0 0 {} {}", width, height);
        self.screen.set_viewbox(viewbox.as_str());
        self.width = ratio * WINDOW_HEIGHT_SS as f32;
        Ok(())
    }

    /// Render the page to the SVG
    fn render_page(&self) {
        log!(RENDER, "render page");
        let mut html = String::new();

        let mut offset_x = STAVE_SPACE; // Stave Margin
        let mut measure = 0;
        loop {
            let mut page = Page::new();
            let width = self.render_measure(measure, offset_x, &mut page);
            html.push_str(&String::from(page));
            log!(RENDER, "measure: {} width {}", measure, width);
            offset_x += width;
            if offset_x >= (self.width * STAVE_SPACE as f32) as i32 {
                break;
            }
            measure += 1;
        }

        let page = self.screen.element_by_id("page").unwrap();
        page.set_inner_html(&html);
    }

    /// Render one measure
    fn render_measure(
        &self,
        measure: u16,
        offset_x: i32,
        page: &mut Page,
    ) -> i32 {
        let offset_y = 0;
        let bar_id = &format!("m{}", measure);
        let trans = &format!("translate({} {})", offset_x, offset_y);

        let mut g = page.frag::<G>();
        g.id(bar_id).transform(trans);

        let high = "C4".parse::<Pitch>().unwrap().visual_distance();
        let low = "C4".parse::<Pitch>().unwrap().visual_distance();

        // Alto clef has 0 steps offset
        let mut bar =
            BarElem::new(Stave::new(5, Steps(4), Steps(0)), high, low);
        bar.add_markings(
            &self.meta,
            &self.program.scof,
            &self.program.cursor,
            measure,
        );
        page.raw(bar.to_string());
        page.close(); // g
        bar.width
    }
}
