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

#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    elapsed: Duration,
    enabled: bool,
}

impl Stopwatch {
    pub const fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
            enabled: true,
        }
    }

    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn advance_by(&mut self, delta: Duration) {
        self.elapsed += delta * (self.enabled as _);
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OneShotTimer {
    stopwatch: Stopwatch,
    deadline: Duration,
}

impl OneShotTimer {
    pub const fn new(deadline: Duration) -> Self {
        Self {
            stopwatch: Stopwatch::new(),
            deadline,
        }
    }

    pub const fn elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub const fn enabled(&self) -> bool {
        self.stopwatch.enabled()
    }

    pub const fn deadline(&self) -> Duration {
        self.deadline
    }

    pub fn is_running(&self) -> bool {
        self.enabled() && !self.expired()
    }

    pub fn expired(&self) -> bool {
        self.stopwatch.elapsed() >= self.deadline
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.stopwatch.set_enabled(enabled);
    }

    pub fn advance_by(&mut self, delta: Duration) {
        self.stopwatch.advance_by(delta * ((!self.expired()) as _));
    }
}
