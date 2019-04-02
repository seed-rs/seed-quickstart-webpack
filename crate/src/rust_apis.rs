use seed;
use wasm_bindgen::prelude::*;
use crate::app;
use crate::set_panic_hook;
use crate::register_custom_events;

// Called by our TS entry point to run the application.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let state = seed::App::build(app::Model::default(), app::update, app::view)
        .window_events(app::window_events)
        .finish()
        .run();

    register_custom_events(app::custom_events, state);

    Ok(())
}