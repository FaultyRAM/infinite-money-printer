// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy_ecs::prelude::*;

#[derive(Clone, Copy, Debug, Resource)]
pub struct UpdateInterval {
    timestamp: f64,
    delta: f64,
}

impl UpdateInterval {
    pub const fn new(timestamp: f64) -> Self {
        Self {
            timestamp,
            delta: 0f64,
        }
    }

    pub fn advance(mut self, timestamp: f64) -> Self {
        self.delta = timestamp - self.timestamp;
        self.timestamp = timestamp;
        self
    }

    pub const fn delta(&self) -> f64 {
        self.delta
    }
}
