use crate::app;
use crate::seed_helpers;
use crate::set_panic_hook;
use seed;
use wasm_bindgen::prelude::*;

// Called by our TS entry point to run the application.
// This function is called from /entries/index.ts
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let state = seed::App::build(
        app::Model::default(),
        seed_helpers::update_wrapper_with_raf,
        app::view,
    )
    .window_events(app::window_events)
    .finish()
    .run();

    seed_helpers::register_custom_events(state);

    Ok(())
}

#[wasm_bindgen]
#[derive(Clone, EnumIter)]
pub enum CustomEventId {
    NoOp,
    OnRequestAnimationFrame, // system event - don't modify
    OnClockTick,
}
