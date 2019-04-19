use crate::app;
use crate::rust_apis;
use seed;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

// Create a new event listener for each rust_apis::CustomEventId item
pub fn register_custom_events(state: seed::App<app::Msg, app::Model>) {
    let window = web_sys::window().expect("should have a window in this context");

    for (index, custom_event_id) in rust_apis::CustomEventId::iter().enumerate() {
        let state = state.clone();

        //https://rustwasm.github.io/wasm-bindgen/examples/closures.html
        let callback = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            state.update(app::Msg::OnCustomEvent(
                custom_event_id.clone(),
                event.detail(),
            ));
        }) as Box<dyn Fn(web_sys::CustomEvent)>);

        window.add_event_listener_with_callback(
            index.to_string().as_str(),
            callback.as_ref().unchecked_ref(),
        );
        callback.forget();
    }
}
