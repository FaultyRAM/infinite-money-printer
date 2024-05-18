// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod credits;
mod game_state;
mod time;
mod ui;

use game_state::GameState;
use std::sync::{Mutex, OnceLock};
use time::Timestamp;
use wasm_bindgen::prelude::*;

static GAME_STATE: OnceLock<Mutex<GameState>> = OnceLock::new();

fn request_animation_frame(f: impl 'static + Fn(f64)) {
    let update_cb: Closure<dyn Fn(f64)> = Closure::new(f);
    let window = web_sys::window().unwrap();
    let _ = window
        .request_animation_frame(update_cb.into_js_value().unchecked_ref())
        .unwrap();
}

fn update(timestamp: f64) {
    let timestamp = Timestamp::at(timestamp);
    let game_state = GAME_STATE.get_or_init(|| Mutex::new(GameState::new(timestamp)));
    game_state.lock().unwrap().update(timestamp);
    request_animation_frame(update);
}

#[wasm_bindgen(start)]
fn run() {
    request_animation_frame(update);
}
