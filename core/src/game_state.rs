// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    credits::{update_credits, Credits},
    ui::update_ui,
};
use bevy_ecs::prelude::*;
use bigdecimal::BigDecimal;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, Resource)]
pub struct UpdateInterval {
    pub timestamp: f64,
    pub delta: f64,
}

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
        world.insert_resource(UpdateInterval {
            timestamp,
            delta: 0f64,
        });
        world.insert_resource(Credits(BigDecimal::from(0)));
        schedule.add_systems((update_credits, update_ui).chain());
        Self { world, schedule }
    }

    pub fn update(&mut self, timestamp: f64) {
        let mut update_interval: Mut<UpdateInterval> = self.world.resource_mut();
        update_interval.delta = timestamp - update_interval.timestamp;
        update_interval.timestamp = timestamp;
        self.schedule.run(&mut self.world);
    }
}
