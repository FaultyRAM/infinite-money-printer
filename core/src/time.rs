// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use bevy_ecs::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Timestamp(f64);

impl Timestamp {
    pub fn now() -> Self {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.timeline().current_time())
            .map(Self::at)
            .unwrap()
    }

    pub fn at(millis: f64) -> Self {
        assert!(millis.is_normal() && millis.is_sign_positive());
        Self(millis)
    }

    pub fn duration_since(&self, earlier: Self) -> Duration {
        assert!(self.0 >= earlier.0);
        self.elapsed() - earlier.elapsed()
    }

    pub fn elapsed(self) -> Duration {
        Duration::from_secs_f64(self.0 / 1000_f64)
    }
}

#[derive(Clone, Copy, Debug, Resource)]
pub struct Time<T> {
    timestamp: Timestamp,
    delta: Duration,
    #[allow(dead_code)]
    scale: T,
}

impl<T: Default> Time<T> {
    pub fn now() -> Self {
        Self::at(Timestamp::now())
    }

    pub fn at(timestamp: Timestamp) -> Self {
        Self {
            timestamp,
            delta: Duration::ZERO,
            scale: T::default(),
        }
    }

    pub const fn delta(&self) -> Duration {
        self.delta
    }
}

impl<T: Default> Default for Time<T> {
    fn default() -> Self {
        Self::now()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Real;

impl Time<Real> {
    pub fn advance_to(&mut self, timestamp: Timestamp) {
        self.delta = timestamp.duration_since(self.timestamp);
        self.timestamp = timestamp;
    }
}
