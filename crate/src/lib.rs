use seed;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

mod app;
mod rust_apis;
mod ts_apis;

cfg_if::cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// @TODO refactor / move to another file
pub fn register_custom_events(
    custom_events: app::CustomEvents<app::Msg, app::Model>,
    state: seed::App<app::Msg, app::Model>,
) {
    let window = web_sys::window().expect("should have a window in this context");
    //https://rustwasm.github.io/wasm-bindgen/examples/closures.html

    let model = state.data.model.borrow();

    let custom_events_pairs = custom_events(&model);

    for pair in custom_events_pairs {
        let custom_event_id: &str = pair.0;
        let custom_event_msg: app::MsConstructor<app::Msg> = pair.1;

        let state = state.clone();
        let callback = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| {
            state.update(custom_event_msg(event.detail()))
        }) as Box<dyn Fn(web_sys::CustomEvent)>);

        window.add_event_listener_with_callback(custom_event_id, callback.as_ref().unchecked_ref());
        callback.forget();
    }
}
