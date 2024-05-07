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
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameState {
    world: World,
    schedule: Schedule,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(timestamp: f64) -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource(UpdateInterval::new(timestamp));
        world.insert_resource(Credits::new());
        schedule.add_systems((update_credits, update_ui).chain());
        Self { world, schedule }
    }

    pub fn update(&mut self, timestamp: f64) {
        let mut update_interval: Mut<UpdateInterval> = self.world.resource_mut();
        *update_interval = update_interval.advance(timestamp);
        self.schedule.run(&mut self.world);
    }
}
