// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod credits;
mod game_state;
mod time;
mod ui;

use game_state::GameState;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Mutex, OnceLock},
};
use wasm_bindgen::prelude::*;

static GAME_STATE: OnceLock<Mutex<GameState>> = OnceLock::new();

fn request_animation_frame(f: &Closure<dyn Fn(f64)>) {
    let window = web_sys::window().unwrap();
    let _ = window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
fn run() {
    GAME_STATE.set(Mutex::new(GameState::new())).unwrap();
    let update_cb = Rc::new(RefCell::new(None));
    let ucb_clone = update_cb.clone();
    *ucb_clone.borrow_mut() = Some(Closure::new(move |timestamp| {
        GAME_STATE.get().unwrap().lock().unwrap().update(timestamp);
        request_animation_frame(update_cb.borrow().as_ref().unwrap());
    }));
    request_animation_frame(ucb_clone.borrow().as_ref().unwrap());
}
