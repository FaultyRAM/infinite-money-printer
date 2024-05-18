// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::time::{OneShotTimer, Real, Time};
use bevy_ecs::prelude::*;
use bigdecimal::{BigDecimal, RoundingMode};
use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

#[derive(Clone, Debug, Resource)]
pub struct Credits(pub BigDecimal);

impl Credits {
    pub fn new() -> Self {
        Self(BigDecimal::from(0))
    }
}

#[derive(Clone, Copy, Debug, Resource)]
pub struct PrintTimer(pub OneShotTimer);

impl PrintTimer {
    pub fn new() -> Self {
        let mut timer = OneShotTimer::new(Duration::from_millis(5000));
        timer.set_enabled(false);
        Self(timer)
    }
}

impl Display for Credits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0.with_scale_round(0, RoundingMode::Floor), f)
    }
}

pub fn update_credits(
    real_time: Res<Time<Real>>,
    mut print_timer: ResMut<PrintTimer>,
    mut credits: ResMut<Credits>,
) {
    print_timer.0.advance_by(real_time.delta());
    if print_timer.0.expired() {
        *print_timer = PrintTimer::new();
        credits.0 += 1;
    }
}
