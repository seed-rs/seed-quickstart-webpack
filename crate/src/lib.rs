use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use seed;
use web_sys;

mod app;
mod ts_apis;
mod rust_apis;

type CustomEvents<Ms, Mdl> = fn(&Mdl) -> Vec<Ms>;

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

// @TODO use custom_events
pub fn register_custom_events(custom_events: CustomEvents<app::Msg, app::Model>, state: seed::App<app::Msg, app::Model>) {
    let window = web_sys::window().expect("should have a window in this context");
    //https://rustwasm.github.io/wasm-bindgen/examples/closures.html
    let a = Closure::wrap(Box::new(move |event: web_sys::CustomEvent| state.update(app::Msg::ClockTick(event.detail()))) as Box<dyn Fn(web_sys::CustomEvent)>);
    window.add_event_listener_with_callback("onclocktick", a.as_ref().unchecked_ref());
    a.forget();
}