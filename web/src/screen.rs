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

use std::future::Future;
use std::task::{Waker, Context, Poll};
use std::cell::RefCell;
use std::pin::Pin;
use std::sync::atomic::{AtomicU32, Ordering, AtomicBool};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::convert::FromWasmAbi;
use web_sys::UiEvent;

const SVGNS: Option<&str> = Some("http://www.w3.org/2000/svg");

static WIDTH: AtomicU32 = AtomicU32::new(0);
static HEIGHT: AtomicU32 = AtomicU32::new(0);
static RESIZED: AtomicBool = AtomicBool::new(false);

thread_local! {
    static WAKER: RefCell<Option<Waker>> = RefCell::new(None);
}

/// Graphical screen.
pub struct Screen {
    window: web_sys::Window,
    document: web_sys::Document,
    svg: web_sys::Element,
}

impl Screen {
    /// Create a new `Screen`
    pub fn new() -> Option<Self> {
        // Get the javascript window.
        let window = web_sys::window()?;
        // Get the DOM Document.
        let document = window.document()?;
        // Get the SVG element.
        let svg = document
            .get_elements_by_tag_name("svg")
            .get_with_index(0)
            .unwrap();

        // Return Screen.
        Some(Screen {
            window,
            document,
            svg,
        })
    }

    /// Set the screen title
    pub fn set_title(&mut self, _title: &str) {
        todo!()
    }

    /// Get a future that returns resize events.
    pub fn resize(&mut self) -> impl Future<Output=(u32, u32)> + Unpin {
        let svg = self.svg.clone();

        self.on_event("resize", move |_ui_event: UiEvent| {
            use super::INFO;
            super::log!(INFO, "{}", svg.client_width());
            super::log!(INFO, "{}", svg.client_height());
            // Resize.
            WIDTH.store(svg.client_width() as u32, Ordering::SeqCst);
            HEIGHT.store(svg.client_height() as u32, Ordering::SeqCst);
            RESIZED.store(true, Ordering::SeqCst);
            // Wake the waker.
            WAKER.with(|w| {
                let waker = w.borrow_mut().take();
                if let Some(wk) = waker {
                    wk.wake_by_ref()
                }
            });
        });
        ResizeEvent
    }

    /// Get the size.
    pub fn size(&self) -> (u32, u32) {
        (self.svg.client_width() as u32, self.svg.client_height() as u32)
    }

    /// Register a javascript global event handler.
    fn on_event<E, F>(&mut self, name: &str, closure: F)
        where E: FromWasmAbi + 'static, F: Fn(E) + 'static
    {
        #[allow(trivial_casts)] // Actually needed here.
        let e: Closure<dyn Fn(E)> = Closure::wrap(Box::new(closure));
        self.window
            .add_event_listener_with_callback(name, e.as_ref().unchecked_ref())
            .expect("Failed to register event");
        e.forget();
    }
    
    /// Set SVG viewbox.
    pub fn viewbox(&mut self, vbox: &str) {
        self.svg.set_attribute("viewBox", vbox).expect("Failed to set attrib");
    }
    
    /// Create a rectangle object.
    pub fn new_rect(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let rect = self.document.create_element_ns(SVGNS, "rect").unwrap();
        rect.set_attribute_ns(None, "x", &x.to_string()).unwrap();
        rect.set_attribute_ns(None, "y", &y.to_string()).unwrap();
        rect.set_attribute_ns(None, "width", &w.to_string()).unwrap();
        rect.set_attribute_ns(None, "height", &h.to_string()).unwrap();
    
        Rect(rect)
    }

    /// Create a new path.
    pub fn new_path(&self, data: &str) -> Path {
        let path = self.document.create_element_ns(SVGNS, "path").unwrap();
        path.set_attribute_ns(None, "d", data).unwrap();
        Path(path)
    }

    /// Create a new use.
    pub fn new_use(&self, x: f32, y: f32, id: &str) -> Use {
        let stamp = self.document.create_element_ns(SVGNS, "use").unwrap();
        stamp.set_attribute_ns(None, "x", &x.to_string()).unwrap();
        stamp.set_attribute_ns(None, "y", &y.to_string()).unwrap();
        stamp.set_attribute_ns(None, "href", id).unwrap();

        Use(stamp)
    }
    
    /// Create a new group.
    pub fn new_group(&self) -> Group {
        let group = self.document.create_element_ns(SVGNS, "g").unwrap();
        Group(group)
    }
    
    /// Add SVG element.
    pub fn append_child(&self, element: web_sys::Element) {
        self.svg.append_child(&element).expect("Failed to append child");
    }
    
    pub fn set_svg(&self, svg: &str) {
        self.svg.set_inner_html(&svg);
    }
    
    pub fn element_by_id(&self, id: &str) -> Option<web_sys::Element> {
        self.document.get_element_by_id(id)
    }
}

pub struct Group(pub web_sys::Element);

pub struct Rect(pub web_sys::Element);

pub struct Path(pub web_sys::Element);

pub struct Use(pub web_sys::Element);

impl Group {
    pub fn set_id(&mut self, id: &str) {
        self.0.set_attribute_ns(None, "id", &id.to_string()).unwrap();
    }
    
    pub fn set_transform(&mut self, trans: &str) {
        self.0.set_attribute_ns(None, "transform", trans).unwrap();
    }
}

impl Rect {
    pub fn set_id(&mut self, id: &str) {
        self.0.set_attribute_ns(None, "id", &id.to_string()).unwrap();
    }

    pub fn set_rx(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "rx", &v.to_string()).unwrap();
    }

    pub fn set_ry(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "ry", &v.to_string()).unwrap();
    }

    pub fn set_fill(&mut self, v: &str) {
        self.0.set_attribute_ns(None, "fill", v).unwrap();
    }

    pub fn set_x(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "x", &v.to_string()).unwrap();
    }

    pub fn set_y(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "y", &v.to_string()).unwrap();
    }

    pub fn set_width(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "width", &v.to_string()).unwrap();
    }

    pub fn set_height(&mut self, v: f32) {
        self.0.set_attribute_ns(None, "height", &v.to_string()).unwrap();
    }
}

struct ResizeEvent;

impl Future for ResizeEvent {
    type Output = (u32, u32);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        WAKER.with(|waker| {
            let ret = if RESIZED.load(Ordering::SeqCst) {
                let width = WIDTH.load(Ordering::SeqCst);
                let height = HEIGHT.load(Ordering::SeqCst);
                RESIZED.store(false, Ordering::SeqCst);
                Poll::Ready((width, height))
            } else {
                Poll::Pending
            };
            *waker.borrow_mut() = Some(cx.waker().clone());
            ret
        })
    }
}
