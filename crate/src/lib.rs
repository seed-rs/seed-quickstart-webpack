// import macros (e.g. button!)
use seed::*;
use seed::prelude::*;

// Model

struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.val += 1,
    }
    Render.into()
}


// View

fn view(model: &Model) -> El<Msg> {
    button![ 
        simple_ev(Ev::Click, Msg::Increment), 
        format!("Hello, World × {}", model.val) 
    ]
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() {
    set_panic_hook();

    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}

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
