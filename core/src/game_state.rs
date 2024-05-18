// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    credits::{update_credits, Credits},
    time::{Real, Time, Timestamp},
    ui::update_ui,
};
use bevy_ecs::prelude::*;
use std::fmt::{self, Debug, Formatter};

pub struct GameState {
    world: World,
    schedule: Schedule,
}

impl GameState {
    pub fn new(timestamp: Timestamp) -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource::<Time<Real>>(Time::at(timestamp));
        world.insert_resource(Credits::new());
        schedule.add_systems((update_credits, update_ui).chain());
        Self { world, schedule }
    }

    pub fn update(&mut self, timestamp: Timestamp) {
        let mut real_time: Mut<Time<Real>> = self.world.resource_mut();
        real_time.advance_to(timestamp);
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
