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

#![recursion_limit = "128"]

use cala::{info, note};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

use stdweb::js;
use stdweb::traits::*;
use stdweb::web::{
    document,
    event::{
        ContextMenuEvent, KeyDownEvent, KeyUpEvent, MouseWheelEvent,
        ResizeEvent,
    },
    window, IEventTarget,
};

use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use scof::{Cursor, Fraction, Pitch, Steps};
use scorefall_studio::Program;
use staverator::{BarElem, Element, Stave};

mod input;

use input::*;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const ZOOM_LEVEL: f64 = 2.0;
const SCALEDOWN: f64 = 50_000.0 / ZOOM_LEVEL;
const SVGNS: &str = "http://www.w3.org/2000/svg";

struct State {
    program: Program,
    time_old: f64,
    #[allow(unused)] // FIXME: Implement commands.
    command: String,
    input: InputState,
    svg: stdweb::web::Element,
}

impl State {
    /// Create a new state
    fn new(svg: stdweb::web::Element) -> State {
        State {
            program: Program::new(),
            time_old: 0.0,
            command: "".to_string(),
            input: InputState::new(),
            svg,
        }
    }

    /// Resize the SVG
    fn resize(&self) -> Result<()> {
        note!("resize");
        let svg = &self.svg;
        let viewbox = js! {
            var ratio = @{svg}.clientHeight / @{svg}.clientWidth;
            return "0 0 " + @{SCALEDOWN} + " " + @{SCALEDOWN} * ratio;
        };
        if let Some(vb) = viewbox.as_str() {
            svg.set_attribute("viewBox", vb)?;
        }
        Ok(())
    }

    fn process_input(&mut self, time: f64) {
        let _dt = (time - self.time_old) as f32;
        self.time_old = time;

        if self.input.has_input {
            if self.input.press(Key::Left) {
                self.program.left();
                self.render_measures();
            }
            if self.input.press(Key::Right) {
                self.program.right();
                self.render_measures();
            }
            if self.input.held(Key::LeftShift)
                || self.input.held(Key::RightShift)
            {
                if self.input.press(Key::J) {
                    self.program.down_half_step();
                    self.render_measures();
                }
                if self.input.press(Key::K) {
                    self.program.up_half_step();
                    self.render_measures();
                }
            } else {
                if self.input.press(Key::J) {
                    self.program.down_step();
                    self.render_measures();
                }
                if self.input.press(Key::K) {
                    self.program.up_step();
                    self.render_measures();
                }
            }
            // Note Lengths
            if self.input.press(Key::Numpad0) {
                self.program.set_dur(Fraction::new(1, 128));
                self.render_measures();
            } else if self.input.press(Key::Numpad1) {
                self.program.set_dur(Fraction::new(1, 64));
                self.render_measures();
            } else if self.input.press(Key::Y) || self.input.press(Key::Numpad2)
            {
                self.program.set_dur(Fraction::new(1, 32));
                self.render_measures();
            } else if self.input.press(Key::S) || self.input.press(Key::Numpad3)
            {
                self.program.set_dur(Fraction::new(1, 16));
                self.render_measures();
            } else if self.input.press(Key::T) || self.input.press(Key::Numpad4)
            {
                self.program.set_dur(Fraction::new(1, 8));
                self.render_measures();
            } else if self.input.press(Key::Q) || self.input.press(Key::Numpad5)
            {
                self.program.set_dur(Fraction::new(1, 4));
                self.render_measures();
            } else if self.input.press(Key::H) || self.input.press(Key::Numpad6)
            {
                self.program.set_dur(Fraction::new(1, 2));
                self.render_measures();
            } else if self.input.press(Key::W) || self.input.press(Key::Numpad7)
            {
                self.program.set_dur(Fraction::new(1, 1));
                self.render_measures();
            } else if self.input.press(Key::Numpad8) {
                self.program.set_dur(Fraction::new(2, 1));
                self.render_measures();
            } else if self.input.press(Key::Numpad9) {
                self.program.set_dur(Fraction::new(4, 1));
                self.render_measures();
            } else if self.input.press(Key::Period)
                || self.input.press(Key::NumpadDot)
            {
                self.program.dotted();
                self.render_measures();
            } /*else if self.input.press(Key::T)  || self.input.press(Key::Numpad0) {
                  self.program.tuplet();
                  self.render_measures();
              } */
        }

        self.input.reset();
    }

    fn run(time: f64, rc: Rc<RefCell<Self>>) {
        rc.borrow_mut().process_input(time);

        window().request_animation_frame(move |time| {
            Self::run(time, rc.clone());
        });
    }

    /// Initialize the score SVG
    fn initialize_score(&self) -> Result<()> {
        let page = document().create_element_ns(SVGNS, "g")?;
        page.set_attribute("id", "page")?;
        let svg = &self.svg;
        js! {
            @{svg}.innerHTML = "";
            @{svg}.appendChild(@{page});
        };
        Ok(())
    }

    /// Render the defs to the SVG
    fn render_defs(&self) -> Result<()> {
        let svg = &self.svg;
        let defs = document().create_element_ns(SVGNS, "defs")?;

        for path in staverator::bravura() {
            let id = path.id.unwrap();
            let shape = document().create_element_ns(SVGNS, "path")?;
            shape.set_attribute("d", &path.d)?;
            shape.set_attribute("id", &id)?;
            defs.append_child(&shape);
        }
        js! {
            @{svg}.appendChild(@{&defs});
        }
        Ok(())
    }

    /// Render the score
    fn render_score(&self) -> Result<()> {
        self.initialize_score()?;
        self.resize()?;
        self.render_defs()?;
        self.render_measures();
        Ok(())
    }

    /// Render the measures to the SVG
    fn render_measures(&self) {
        note!("render measures");
        let svg = &self.svg;
        js! {
            var page = @{svg}.getElementById("page");
            page.innerHTML = "";
        };

        let mut offset_x = 0;
        for measure in 0..9 {
            let width = self.render_measure(measure, offset_x);
            note!("measure: {}  width {}", measure, width);
            offset_x += width;
        }
    }

    /// Render one measure
    fn render_measure(&self, measure: u16, offset_x: i32) -> i32 {
        // FIXME: iterate through channels
        let offset_y = 0;
        let bar_id = &format!("m{}", measure);
        let trans = &format!("translate({} {})", offset_x, offset_y);
        let svg = &self.svg;
        let bar_g = js! {
            var page = @{svg}.getElementById("page");
            var old_g = @{svg}.getElementById(@{bar_id});
            var bar_g = document.createElementNS(@{SVGNS}, "g");
            bar_g.setAttributeNS(null, "id", @{bar_id});
            bar_g.setAttributeNS(null, "transform", @{trans});
            if (old_g !== null) {
                old_g.replaceWith(bar_g);
            } else {
                page.appendChild(bar_g);
            }
            return bar_g;
        };

        let high = "C4".parse::<Pitch>().unwrap().visual_distance();
        let low = "C4".parse::<Pitch>().unwrap().visual_distance();

        let mut bar_width = 0;
        let mut ypos = Steps(0);
        for i in 0..2 { // FIXME
        let mut curs = Cursor::new(0 /*mvmt*/, measure, i /*chan*/, 0 /*marking*/);
        // Alto clef has 0 steps offset
        let mut bar = BarElem::new(Stave::new(5, Steps(4), ypos), high, low);
        if curs == self.program.cursor.first_marking() {
            bar.add_cursor(&self.program.scof, &self.program.cursor);
        }
        if measure == 0 {
            bar.add_signature();
        }
        bar.add_markings(&self.program.scof, &mut curs);
        bar.add_stave();

        for elem in bar.elements {
            if let Some(e) = create_elem(elem) {
                js! { @{&bar_g}.appendChild(@{e}); }
            }
        }
        bar_width = bar.width;
        let stave = Stave::new(5, Steps(4), ypos);
        ypos = ypos + stave.height_steps() + Steps(12); // Margin above & below
        }
        bar_width
    }
}

/// Create DOM element from a staverator Element
fn create_elem(elem: Element) -> Option<stdweb::Value> {
    match elem {
        Element::Rect(r) => Some(js! {
            var rect = document.createElementNS(@{SVGNS}, "rect");
            rect.setAttributeNS(null, "x", @{r.x});
            rect.setAttributeNS(null, "y", @{r.y});
            rect.setAttributeNS(null, "width", @{r.width});
            rect.setAttributeNS(null, "height", @{r.height});
            var rx = @{r.rx};
            if (rx !== null) {
                rect.setAttributeNS(null, "rx", rx);
            }
            var ry = @{r.ry};
            if (ry !== null) {
                rect.setAttributeNS(null, "ry", ry);
            }
            rect.setAttributeNS(null, "fill", @{r.fill});
            return rect;
        }),
        Element::Use(u) => {
            let xlink = format!("#{:x}", u.id);
            Some(js! {
                var stamp = document.createElementNS(@{SVGNS}, "use");
                stamp.setAttributeNS(null, "x", @{u.x});
                stamp.setAttributeNS(null, "y", @{u.y});
                stamp.setAttributeNS(null, "href", @{xlink});
                return stamp;
            })
        }
        Element::Path(p) => Some(js! {
            var path = document.createElementNS(@{SVGNS}, "path");
            path.setAttributeNS(null, "d", @{p.d});
            return path;
        }),
        _ => None,
    }
}

fn panic_hook(panic_info: &std::panic::PanicInfo) {
    let msg = panic_info.to_string();

    info!("Custom panic: {:?}", msg);
    js! { console.trace() }

    std::process::exit(0);
}

fn main() {
    stdweb::initialize();
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |p| {
        hook(p);
        panic_hook(p);
    }));

    let svg = document().get_element_by_id("canvas").unwrap();
    let state = Rc::new(RefCell::new(State::new(svg)));

    // FIXME: Use this.
    let _prompt: stdweb::web::Element =
        document().get_element_by_id("prompt").unwrap();

    window().add_event_listener(enclose!( (state) move |_: ResizeEvent| {
        state.borrow().resize().unwrap();
    }));

    window().add_event_listener(
        enclose!( (/*state*/) move |event: ContextMenuEvent| {
        //        js! {
        //            alert("success!");
        //        }
                event.prevent_default();
            }),
    );

    // CTRL-W, CTRL-Q, CTRL-T, CTRL-N aren't picked up by this (Tested chromium,
    // firefox).
    window().add_event_listener(enclose!( (state) move |event: KeyDownEvent| {
        let is = &mut state.borrow_mut().input;
        let key = event.key();
        let code = event.code();

        if code != "F11" {
            is.update(key, code, event.is_composing(), true);
            event.prevent_default();
        }
    }));
    window().add_event_listener(enclose!( (state) move |event: KeyUpEvent| {
        let is = &mut state.borrow_mut().input;
        let key = event.key();
        let code = event.code();

        if code != "F11" {
            is.update(key, code, event.is_composing(), false);
            event.prevent_default();
        }
    }));

    window().add_event_listener(
        enclose!( (/*state*/) move |event: MouseWheelEvent| {
        //        js! {
        //            alert("keydown!");
        //        }
            event.prevent_default();
        }),
    );

    note!("YA");

    state.borrow().render_score().unwrap();

    State::run(0.0, state.clone());
    stdweb::event_loop();
}
