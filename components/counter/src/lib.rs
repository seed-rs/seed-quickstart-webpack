use atomic_refcell::{AtomicRefCell};
use wasm_bindgen::prelude::*;
use counter_component_interface::{Id, Message};

struct Model {
    counters: Vec<Counter>,
}

static MODEL: AtomicRefCell<Model> = AtomicRefCell::new({
    Model {
        counters: Vec::new(),
    }
});

struct Counter {
    count: i32,
}

#[wasm_bindgen]
// NOTE: `new` can't be used because of `TS1359: Identifier expected. 'new' is a reserved word.` in `index.d.ts`.
pub fn new_counter() -> Id {
    MODEL.borrow_mut().counters.push(Counter {count: 0});
    MODEL.borrow().counters.len() - 1
}

#[wasm_bindgen]
pub fn update(id: Id, message: Message) {
    match message {
        Message::Increment => MODEL.borrow_mut().counters[id].count += 1,
        Message::Decrement => MODEL.borrow_mut().counters[id].count -= 1,
    }
}

#[wasm_bindgen]
pub fn view(id: Id) -> String {
    format!("Count: {}", MODEL.borrow().counters[id].count)
}
