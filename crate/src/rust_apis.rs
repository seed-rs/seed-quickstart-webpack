use seed;
use seed::prelude::wasm_bindgen;
use crate::app;
use crate::set_panic_hook;

// Called by our TS entry point to run the application.
#[wasm_bindgen]
pub fn run() {
    set_panic_hook();

    seed::App::build(app::Model::default(), app::update, app::view)
        .finish()
        .run();
}