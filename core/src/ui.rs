// SPDX-FileCopyrightText: 2024 FaultyRAM <316014+FaultyRAM@users.noreply.github.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    credits::{Credits, PrintTimer},
    GAME_STATE,
};
use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Event;

const ID_UI_CREDITS: &str = "ui-credits";
const ID_UI_MAIN: &str = "ui-main";
const ID_UI_PRINT_MONEY: &str = "ui-print-money";
const ID_UI_PRINT_PROGRESS: &str = "ui-print-progress";

pub fn init_ui() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let ui_print_money = document.get_element_by_id(ID_UI_PRINT_MONEY).unwrap();
    let cb: Closure<dyn Fn(Event)> = Closure::new(ui_print_money_onclick);
    ui_print_money
        .add_event_listener_with_callback("click", cb.into_js_value().unchecked_ref())
        .unwrap();
}

pub fn update_ui(credits: Res<Credits>, print_timer: Res<PrintTimer>) {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .unwrap();
    let ui_credits = document.get_element_by_id(ID_UI_CREDITS).unwrap();
    ui_credits.set_text_content(Some(&credits.to_string()));
    let ui_print_progress = document.get_element_by_id(ID_UI_PRINT_PROGRESS).unwrap();
    let print_progress_max = print_timer.0.deadline().as_secs_f64();
    let print_progress_value = print_timer
        .0
        .elapsed()
        .as_secs_f64()
        .min(print_progress_max);
    ui_print_progress
        .set_attribute("value", &print_progress_value.to_string())
        .unwrap();
    ui_print_progress
        .set_attribute("max", &print_progress_max.to_string())
        .unwrap();
    let ui_print_money = document.get_element_by_id(ID_UI_PRINT_MONEY).unwrap();
    ui_print_money
        .toggle_attribute_with_force("disabled", print_timer.0.is_running())
        .unwrap();
    let ui_main = document.get_element_by_id(ID_UI_MAIN).unwrap();
    ui_main.remove_attribute("hidden").unwrap();
}

fn ui_print_money_onclick(_event: Event) {
    GAME_STATE.get().unwrap().lock().unwrap().print_money();
}
