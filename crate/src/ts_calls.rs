pub mod helpers {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/ts/helpers.ts")]
    extern "C" {
        pub fn get_random_number(min: i32, max: i32) -> i32;
    }
}