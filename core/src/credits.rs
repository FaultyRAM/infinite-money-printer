// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::game_state::UpdateInterval;
use bevy_ecs::prelude::*;
use bigdecimal::BigDecimal;

#[derive(Clone, Debug, Resource)]
pub struct Credits(pub BigDecimal);

pub fn update_credits(interval: Res<UpdateInterval>, mut credits: ResMut<Credits>) {
    let base_production_rate = BigDecimal::from(1);
    let delta = BigDecimal::try_from(interval.delta / 1000.0).unwrap();
    credits.0 += base_production_rate * delta;
}
