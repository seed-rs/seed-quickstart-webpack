use crate::app;
use crate::set_panic_hook;
use seed;
use wasm_bindgen::prelude::*;

// Called by /entries/index.ts to run the application.
// This function is called from /entries/index.ts
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    seed::App::build(
        app::init,
        app::update,
        app::view,
    )
    .window_events(app::window_events)
    .finish()
    .run();

    Ok(())
}
