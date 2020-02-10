use wasm_bindgen::prelude::*;

pub type Id = usize;

#[wasm_bindgen]
pub enum Message {
    Increment,
    Decrement
}
