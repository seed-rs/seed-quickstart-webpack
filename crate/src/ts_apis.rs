// READ https://github.com/rustwasm/wasm-bindgen/blob/master/guide/src/reference/js-snippets.md

// see ./ts/helpers.ts and ./app.rs (ts_apis::helpers::get_random_number)
pub mod helpers {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/ts/helpers.ts")]
    extern "C" {
        pub fn get_random_number(min: i32, max: i32) -> i32;
    }
}
