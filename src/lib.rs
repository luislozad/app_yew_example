#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;

mod core;

use crate::core::models::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}