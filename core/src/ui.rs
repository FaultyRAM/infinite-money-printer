// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::credits::Credits;
use bevy_ecs::prelude::*;

const ID_UI_MAIN: &str = "ui-main";
const ID_UI_CREDITS: &str = "ui-credits";

pub fn update_ui(credits: Res<Credits>) {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .unwrap();
    let ui_credits = document.get_element_by_id(ID_UI_CREDITS).unwrap();
    ui_credits.set_text_content(Some(&credits.to_string()));
    let ui_main = document.get_element_by_id(ID_UI_MAIN).unwrap();
    ui_main.remove_attribute("hidden").unwrap();
}
