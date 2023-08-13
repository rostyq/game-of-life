//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::println;

use wasm_bindgen_test::*;

use game_of_life_wasm::JsWorld;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn run_world() {
    let mut w = JsWorld::random(64, 64, 42, 0.5).unwrap();
    for _ in 0..100 {
        w.update();
    }
}
