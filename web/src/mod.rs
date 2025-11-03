// ScoreFall Ink - Music Composition Software
//
// Copyright © 2019-2025 Jeryn Aldaron Lau <aldaronlau@gmail.com>
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

mod screen;
mod state;

use async_main::LocalSpawner;
use devout::{Tag, log};
use pasts::Loop;

use crate::state::State;

const INFO: Tag = Tag::new("Info");

/// Initialize program state
fn init() -> State {
    let hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |p| {
        hook(p);
        log!(INFO, "ScoreFall Ink panicked!: {:?}", p.to_string());
        web_sys::console::trace_0();
        std::process::exit(0);
    }));

    let mut state = State::new();
    state.render_score();
    state
}

#[async_main::async_main]
async fn main(_spawner: LocalSpawner) {
    let mut state = init();

    Loop::new(&mut state)
        .on(|s| &mut s.input, State::event_input)
        .on(|s| &mut s.resize, State::event_resize)
        .await;
}
