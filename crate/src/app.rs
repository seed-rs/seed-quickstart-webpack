use crate::generated::css_classes::C;
use crate::rust_apis;
use crate::seed_helpers::UpdateReturn;
use crate::ts_apis;
use seed::prelude::*;
use seed::*;
use web_sys;

// Model

pub struct Model {
    pub clicks: i32,
    pub random_number: i32,
    pub clock_time: Option<String>,
    pub should_render_next_frame: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            random_number: ts_apis::helpers::get_random_number(0, 100),
            clock_time: None,
            should_render_next_frame: false,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    OnCustomEvent(rust_apis::CustomEventId, wasm_bindgen::JsValue), // don't modify
    Increment,
    NewRandomNumber,
    KeyPressed(web_sys::KeyboardEvent),
}

// Update

pub fn update(msg: Msg, model: &mut Model) -> UpdateReturn {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0, 100),
        Msg::KeyPressed(ev) => log!(ev.key()),
        Msg::OnCustomEvent(custom_event_id, js_value) => match custom_event_id {
            rust_apis::CustomEventId::NoOp => (),
            // --- system handler - don't modify ---
            rust_apis::CustomEventId::OnRequestAnimationFrame => {
                model.should_render_next_frame = false;
                return UpdateReturn::ForceRenderNow;
            }
            // --- // ---
            rust_apis::CustomEventId::OnClockTick => {
                model.clock_time = js_value.as_string();
            }
        },
    }
    UpdateReturn::Render
}

// View

pub fn view(model: &Model) -> Vec<El<Msg>> {
    vec![div![
        // dots at the top
        class![C.bg_custom],
        div![
            class![
                C.h_screen,
                C.w_screen,
                C.flex,
                C.flex_wrap,
                C.justify_center,
                C.content_center
            ],
            img![attrs! {At::Src => "static/images/quickstart.png";}],
            // button container
            div![
                class![C.flex, C.flex_wrap, C.justify_center, C.content_center],
                // button `clicks`
                button![
                    class![
                        C.mb_8,
                        C.mx_8,
                        C.p_4,
                        C.rounded,
                        C.shadow_md,
                        C.bg_green_200,
                        C.hover__bg_green_400,
                        C.font_bold,
                    ],
                    simple_ev(Ev::Click, Msg::Increment),
                    format!("Clicks: {}", model.clicks)
                ],
                // button `random number`
                button![
                    class![
                        C.mb_8,
                        C.p_4,
                        C.rounded,
                        C.shadow_md,
                        C.bg_blue_200,
                        C.hover__bg_blue_400,
                        C.font_bold
                    ],
                    simple_ev(Ev::Click, Msg::NewRandomNumber),
                    format!("Random number from Typescript: {}", model.random_number)
                ]
            ]
        ],
        // label `press any key`
        div![
            class![
                C.mb_16,
                C.absolute,
                C.inset_x_0,
                C.bottom_0,
                C.text_center,
                C.font_bold,
                C.text_blue_200
            ],
            "Press any key and look at the console log"
        ],
        // time
        div![
            class![
                C.mb_6,
                C.absolute,
                C.inset_x_0,
                C.bottom_0,
                C.text_center,
                C.font_bold,
                C.text_xl,
                C.text_green_400
            ],
            format!(
                "{}",
                if let Some(ref time) = model.clock_time {
                    time
                } else {
                    ""
                }
            )
        ]
    ]]
}

// Subscriptions

pub fn window_events(_model: &Model) -> Vec<dom_types::Listener<Msg>> {
    vec![keyboard_ev("keydown", |ev| Msg::KeyPressed(ev))]
}
