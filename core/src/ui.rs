// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::credits::Credits;
use bevy_ecs::prelude::*;
use bigdecimal::RoundingMode;

pub fn update_ui(credits: Res<Credits>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let ui_credits = document.get_element_by_id("ui-credits").unwrap();
    let credits_str = credits
        .0
        .with_scale_round(0, RoundingMode::Floor)
        .to_string();
    ui_credits.set_text_content(Some(&credits_str));
}
