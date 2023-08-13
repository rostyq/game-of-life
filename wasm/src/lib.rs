use wasm_bindgen::prelude::*;

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use game_of_life_core::{World, patterns::glider};

#[wasm_bindgen(js_name = World)]
pub struct JsWorld(World);

#[wasm_bindgen(js_class = World)]
impl JsWorld {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Self(World::empty(width, height))
    }

    #[wasm_bindgen]
    pub fn random(width: u32, height: u32, seed: u64, prob: f64) -> Result<JsWorld, String> {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        World::random(&mut Xoshiro256StarStar::seed_from_u64(seed), prob, width, height)
            .map_err(|e| e.to_string())
            .map(|w| Self(w))
    }

    #[wasm_bindgen(getter)]
    pub fn population(&self) -> u64 {
        self.0.population()
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.0.size()
    }

    #[wasm_bindgen(getter, js_name = pointer)]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr().cast()
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.0.update();
    }

    #[wasm_bindgen(js_name = glider)]
    pub fn set_glider(&mut self, row: u32, column: u32) {
        self.0.put(row, column, glider());
    }

    #[wasm_bindgen]
    pub fn toggle(&mut self, row: u32, column: u32) {
        if let Some(state) = self.0.get_mut(row, column) {
            state.swap();
        }
    }
}