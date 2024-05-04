// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Core {
    world: World,
    schedule: Schedule,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let world = World::new();
        let schedule = Schedule::default();
        Self { world, schedule }
    }

    pub fn update(&mut self, _delta: f64) {
        self.schedule.run(&mut self.world);
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
