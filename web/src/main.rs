// ScoreFall Ink - Music Composition Software
//
// Copyright © 2019-2021 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright © 2019-2021 Doug P. Lau
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

// bar is a useful musical term
#![allow(clippy::blacklisted_name)]

mod screen;

use screen::{Screen, Rect};

include!("glue.rs");

use cala::log::{Tag, log};
use cala::input::{Input, Key};
use cala::task::{exec, wait};

use std::panic;

use scof::{Cursor, Fraction, Pitch, Steps};
use scorefall_ink::Program;
use staverator::{BarElem, Element, SfFontMetadata, Stave, STAVE_SPACE};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const ZOOM_LEVEL: f32 = 1.0;
// Stave spaces for window height.
const WINDOW_HEIGHT_SS: i32 = 64;
const SCALEDOWN: f32 = (STAVE_SPACE * WINDOW_HEIGHT_SS) as f32 / ZOOM_LEVEL;

const INFO: Tag = Tag::new("Info");
const RENDER: Tag = Tag::new("Render");
const GUI: Tag = Tag::new("Gui");

/// Create DOM element from a staverator Element
fn create_elem(screen: &Screen, elem: Element) -> Option<web_sys::Element> {
    Some(match elem {
        Element::Rect(r) => {
            let mut rect = screen.new_rect(r.x as f32, r.y as f32, r.width as f32, r.height as f32);
            if let Some(v) = r.rx {
                rect.set_rx(v as f32);
            }
            if let Some(v) = r.ry {
                rect.set_ry(v as f32);
            }
            if let Some(fill) = r.fill {
                rect.set_fill(&fill);
            }
            rect.0
        },
        Element::Use(u) => {
            let id = format!("#{:x}", u.id);
            let stamp = screen.new_use(u.x as f32, u.y as f32, &id);
            stamp.0
        }
        Element::Path(p) => {
            let path = screen.new_path(&p.d);
            path.0
        }
        _ => return None,
    })
}

/// Event handled by the event loop.
enum Event {
    Input(Input),
    Resize((u32, u32)),
}

struct State {
    // The web front-end.
    screen: Screen,
    // The front-end agnostic back-end
    program: Program,
    meta: SfFontMetadata,
    // Window width in Stave Spaces.
    width: f32,
}

impl State {
    /// Create a new state
    fn new() -> State {
        let screen = Screen::new().expect("Failed to create screen");
        let mut cursor = screen.new_rect(0.0, 0.0, 1024.0, 1024.0);
        cursor.set_id("cursor");
        cursor.set_fill("#FF9AF0");
        let (meta, defs) = staverator::modern();
        screen.set_svg(&defs);
        screen.append_child(cursor.0);

        State {
            screen,
            program: Program::new(),
            meta,
            width: 0.0,
        }
    }
    
    /// Event loop.
    fn event(&mut self, event: Event) {
        match event {
            Event::Input(input) => self.event_input(input),
            Event::Resize(size) => self.resize(size).unwrap(),
        }
    }
    
    /// Input handler.
    fn event_input(&mut self, input: Input) {
        match input {
            Input::Key(mods, key, true) if mods.ctrl() && matches!(key, Key::H | Key::Left) => {
                // TODO: Halve duration
            }
            Input::Key(mods, key, true) if mods.ctrl() && matches!(key, Key::J | Key::Down) => {
                self.program.down_half_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.ctrl() && matches!(key, Key::K | Key::Up) => {
                self.program.up_half_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.ctrl() && matches!(key, Key::L | Key::Right) => {
                // TODO: Double duration
            }

            Input::Key(mods, key, true) if mods.alt() && matches!(key, Key::H | Key::Left) => {
                // TODO: Move selection to the left
            }
            Input::Key(mods, key, true) if mods.alt() && matches!(key, Key::J | Key::Down) => {
                self.program.down_quarter_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.alt() && matches!(key, Key::K | Key::Up) => {
                self.program.up_quarter_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.alt() && matches!(key, Key::L | Key::Right) => {
                // TODO: Move selection to the right
            }
            Input::Key(mods, key, true) if mods.shift() && matches!(key, Key::H | Key::Left) => {
                // TODO: Select left
            }
            Input::Key(mods, key, true) if mods.shift() && matches!(key, Key::J | Key::Down) => {
                // TODO: Select down
            }
            Input::Key(mods, key, true) if mods.shift() && matches!(key, Key::K | Key::Up) => {
                // TODO: Select up
            }
            Input::Key(mods, key, true) if mods.shift() && matches!(key, Key::L | Key::Right) => {
                // TODO: Select right
            }

            Input::Key(mods, key, true) if mods.none() && matches!(key, Key::H | Key::Left) => {
                self.program.left();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.none() && matches!(key, Key::J | Key::Down) => {
                self.program.down_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.none() && matches!(key, Key::K | Key::Up) => {
                self.program.up_step();
                self.render_measures();
            }
            Input::Key(mods, key, true) if mods.none() && matches!(key, Key::L | Key::Right) => {
                self.program.right();
                self.render_measures();
            }
            Input::Key(mods, Key::One, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 64));
                self.render_measures();
            }
            Input::Key(mods, Key::Two, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 32));
                self.render_measures();
            }
            Input::Key(mods, Key::Three, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 16));
                self.render_measures();
            }
            Input::Key(mods, Key::Four, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 8));
                self.render_measures();
            }
            Input::Key(mods, Key::Five, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 4));
                self.render_measures();
            }
            Input::Key(mods, Key::Six, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 2));
                self.render_measures();
            }
            Input::Key(mods, Key::Seven, true) if mods.none() => {
                self.program.set_dur(Fraction::new(1, 1));
                self.render_measures();
            }
            Input::Key(mods, Key::Eight, true) if mods.none() => {
                self.program.set_dur(Fraction::new(2, 1));
                self.render_measures();
            }
            Input::Key(mods, Key::Nine, true) if mods.none() => {
                self.program.set_dur(Fraction::new(4, 1));
                self.render_measures();
            }
            Input::Key(mods, Key::Period, true) if mods.none() => {
                self.program.dotted();
                self.render_measures();
            }
            _ => { /* ignore all other input */ },
        }
    }

    /// Resize the SVG
    fn resize(&mut self, size: (u32, u32)) -> Result<()> {
        log!(GUI, "Resize {:?}", size);
        let ratio: f32 = size.0 as f32 / size.1 as f32;
        let width = SCALEDOWN * ratio;
        let height = SCALEDOWN;
        let viewbox = format!("0 0 {} {}", width, height);
        self.screen.viewbox(viewbox.as_str());
        self.width = ratio * WINDOW_HEIGHT_SS as f32;
        Ok(())
    }

    /// Initialize the score SVG
    fn initialize_score(&self) -> Result<()> {
        let mut page = self.screen.new_group();
        page.set_id("page");
        self.screen.append_child(page.0);
        Ok(())
    }

    /// Render the score
    fn render_score(&mut self) -> Result<()> {
        self.initialize_score()?;
        self.resize(self.screen.size())?;
        self.render_measures();
        Ok(())
    }

    /// Render the measures to the SVG
    fn render_measures(&self) {
        log!(RENDER, "render measures");
        let page = self.screen.element_by_id("page").unwrap();
        page.set_inner_html("");

        let mut offset_x = STAVE_SPACE; // Stave Margin
        let mut measure = 0;
        'render_measures: loop {
            let width = self.render_measure(measure, offset_x);
            log!(RENDER, "measure: {} width {}", measure, width);
            offset_x += width;
            if offset_x >= (self.width * STAVE_SPACE as f32) as i32 {
                break 'render_measures;
            }
            measure += 1;
        }
    }

    /// Render one measure
    fn render_measure(&self, measure: u16, offset_x: i32) -> i32 {
        let offset_y = 0;
        let bar_id = &format!("m{}", measure);
        let trans = &format!("translate({} {})", offset_x, offset_y);
        let page = self.screen.element_by_id("page").unwrap();
        let old_g = self.screen.element_by_id(bar_id);
        let mut bar_g = self.screen.new_group();
        bar_g.set_id(bar_id);
        bar_g.set_transform(trans);
        let bar_g = if let Some(old_g) = old_g {
            old_g.replace_with_with_node_1(&bar_g.0).unwrap();
            bar_g
        } else {
            page.append_child(&bar_g.0).unwrap();
            bar_g
        };

        let high = "C4".parse::<Pitch>().unwrap().visual_distance();
        let low = "C4".parse::<Pitch>().unwrap().visual_distance();

        let mut curs = Cursor::new(
            0, /*mvmt*/
            measure, 0, /*i chan*/
            0, /*marking*/
        );
        // Alto clef has 0 steps offset
        let mut bar =
            BarElem::new(Stave::new(5, Steps(4), Steps(0)), high, low);
        if let Some((cx, cy, cwidth, cheight)) = bar.add_markings(
            &self.meta,
            &self.program.scof,
            &self.program.cursor,
            &mut curs,
        ) {
            let mut cur = Rect(self.screen.element_by_id("cursor").unwrap());
            cur.set_x((cx + offset_x) as f32);
            cur.set_y(cy as f32);
            cur.set_width(cwidth as f32);
            cur.set_height(cheight as f32);
        }

        for elem in bar.elements {
            if let Some(e) = create_elem(&self.screen, elem) {
                bar_g.0.append_child(&e).unwrap();
            }
        }

        bar.width
    }
}

fn main() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |p| {
        hook(p);
        log!(INFO, "ScoreFall Ink panicked!: {:?}", p.to_string());
        web_sys::console::trace_0();
        std::process::exit(0);
    }));

    let mut state = State::new();
    state.render_score().unwrap();

    let mut input = Input::listener();
    let mut resize = state.screen.resize();

    exec!(state.event(wait! {
        Event::Input((&mut input).await),
        Event::Resize((&mut resize).await),
    }));
}
