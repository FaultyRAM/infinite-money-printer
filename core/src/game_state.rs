// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    credits::{update_credits, Credits},
    timing::UpdateInterval,
    ui::update_ui,
};
use bevy_ecs::prelude::*;
use std::fmt::{self, Debug, Formatter};

pub struct GameState {
    world: World,
    schedule: Schedule,
}

impl GameState {
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

impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameState")
            .field("world", &self.world)
            .finish()
    }
}
