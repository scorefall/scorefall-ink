// ScoreFall Ink - Music Composition Software
//
// Copyright © 2019-2021 Jeron Aldaron Lau <jeronlau@plopgrizzly.com>
// Copyright © 2019-2025 Doug P. Lau
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

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    task::{Context, Poll, Waker},
};

use wasm_bindgen::{JsCast, closure::Closure, convert::FromWasmAbi};
use web_sys::UiEvent;

static WIDTH: AtomicU32 = AtomicU32::new(0);
static HEIGHT: AtomicU32 = AtomicU32::new(0);
static RESIZED: AtomicBool = AtomicBool::new(false);

thread_local! {
    static WAKER: RefCell<Option<Waker>> = RefCell::new(None);
}

/// Graphical screen.
pub struct Screen {
    // Window containing the DOM document
    window: web_sys::Window,
    // DOM Document for the application
    document: web_sys::Document,
    // Root SVG element
    svg: web_sys::Element,
}

impl Screen {
    /// Create a new `Screen`
    pub fn new() -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let svg = document.get_elements_by_tag_name("svg").get_with_index(0)?;
        Some(Screen {
            window,
            document,
            svg,
        })
    }

    /// Get a future that returns resize events.
    pub fn resize(
        &mut self,
    ) -> impl Future<Output = (u32, u32)> + Unpin + use<> {
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
        (
            self.svg.client_width() as u32,
            self.svg.client_height() as u32,
        )
    }

    /// Register a javascript global event handler.
    fn on_event<E, F>(&mut self, name: &str, closure: F)
    where
        E: FromWasmAbi + 'static,
        F: Fn(E) + 'static,
    {
        #[allow(trivial_casts)] // Actually needed here.
        let e: Closure<dyn Fn(E)> = Closure::wrap(Box::new(closure));
        self.window
            .add_event_listener_with_callback(name, e.as_ref().unchecked_ref())
            .expect("Failed to register event");
        e.forget();
    }

    /// Set SVG viewbox.
    pub fn set_viewbox(&mut self, vbox: &str) {
        self.svg
            .set_attribute("viewBox", vbox)
            .expect("Failed to set attrib");
    }

    /// Set the SVG content
    pub fn set_svg(&self, svg: &str) {
        self.svg.set_inner_html(&svg);
    }

    /// Get a DOM element by ID
    pub fn element_by_id(&self, id: &str) -> Option<web_sys::Element> {
        self.document.get_element_by_id(id)
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
