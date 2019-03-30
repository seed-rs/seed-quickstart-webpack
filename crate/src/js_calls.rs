use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/get_random_number.js")]
extern "C" {
    pub fn get_random_number(min: i32, max: i32) -> i32;
}