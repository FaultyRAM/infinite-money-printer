// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::time::{Real, Time};
use bevy_ecs::prelude::*;
use bigdecimal::{BigDecimal, RoundingMode};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Resource)]
pub struct Credits(pub BigDecimal);

impl Credits {
    pub fn new() -> Self {
        Self(BigDecimal::from(0))
    }
}

impl Display for Credits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0.with_scale_round(0, RoundingMode::Floor), f)
    }
}

pub fn update_credits(real_time: Res<Time<Real>>, mut credits: ResMut<Credits>) {
    let base_production_rate = BigDecimal::from(1);
    let delta = BigDecimal::try_from(real_time.delta().as_secs_f64()).unwrap();
    credits.0 += base_production_rate * delta;
}
