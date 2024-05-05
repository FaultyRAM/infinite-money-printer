// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, Resource)]
/// Information about the current update interval.
struct UpdateInterval {
    /// The timestamp associated with the most recently executed update.
    ///
    /// Each time `Core::update` is called, before executing the update schedule, this is set to the
    /// value of the caller-provided timestamp.
    pub timestamp: f64,
    /// The amount of time that has elapsed since the previous update.
    ///
    /// Each time `Core::update` is called, before executing the update schedule, this is set to the
    /// difference of the caller-provided timestamp and the currently stored timestamp.
    pub delta: f64,
}

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

    pub fn update(&mut self, timestamp: f64) {
        let mut update_interval = self.world.get_resource_or_insert_with(|| UpdateInterval {
            timestamp,
            delta: 0.0,
        });
        update_interval.delta = timestamp - update_interval.timestamp;
        update_interval.timestamp = timestamp;
        self.schedule.run(&mut self.world);
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}
