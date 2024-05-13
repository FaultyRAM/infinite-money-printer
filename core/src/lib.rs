// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod credits;
mod timing;
mod ui;

use crate::{
    credits::{update_credits, Credits},
    timing::UpdateInterval,
    ui::update_ui,
};
use bevy_ecs::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    let window = web_sys::window().unwrap();
    let _ = window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
fn run() {
    let timestamp = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.timeline().current_time())
        .unwrap();
    let mut world = World::new();
    let mut schedule = Schedule::default();
    world.insert_resource(UpdateInterval::new(timestamp));
    world.insert_resource(Credits::new());
    schedule.add_systems((update_credits, update_ui).chain());
    let update_cb = Rc::new(RefCell::new(None));
    let ucb_clone = update_cb.clone();
    *ucb_clone.borrow_mut() = Some(Closure::new(move |timestamp| {
        let mut update_interval: Mut<UpdateInterval> = world.resource_mut();
        *update_interval = update_interval.advance(timestamp);
        schedule.run(&mut world);
        request_animation_frame(update_cb.borrow().as_ref().unwrap());
    }));
    request_animation_frame(ucb_clone.borrow().as_ref().unwrap());
}
