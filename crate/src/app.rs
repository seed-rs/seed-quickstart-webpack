use crate::generated::css_classes::C;
use crate::rust_apis;
use crate::ts_apis;
use seed::prelude::*;
use seed::*;
//use wasm_bindgen::JsCast;
use web_sys;

// @TODO refactor / move to another file
pub type CustomEventDetail = JsValue;
pub type MsgConstructor<Msg> = fn(CustomEventDetail) -> Msg;
pub type CustomEvents<Msg, Model> = fn(&Model) -> Vec<(&str, MsgConstructor<Msg>)>;

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

// Update

#[derive(Clone)]
pub enum Msg {
    Increment,
    NewRandomNumber,
    KeyPressed(web_sys::KeyboardEvent),
    OnCustomEvent(rust_apis::CustomEventId, wasm_bindgen::JsValue),
}
// @TODO refactor into two functions
pub fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    let mut should_render = true;
    let mut force_render_now = false;
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0, 100),
        Msg::KeyPressed(ev) => log!(ev.key()),
        Msg::OnCustomEvent(custom_event_id, jsValue) => match custom_event_id {
            rust_apis::CustomEventId::NoOp => (),
            rust_apis::CustomEventId::OnRequestAnimationFrame => {
                model.should_render_next_frame = false;
                force_render_now = true;
            }
            rust_apis::CustomEventId::OnClockTick => {
                model.clock_time = jsValue.as_string();
            }
        },
    }
    if force_render_now {
        Render.into()
    } else {
        if should_render && !model.should_render_next_frame {
            model.should_render_next_frame = true;
            ts_apis::seed_helpers::callRequestAnimationFrame();
        }
        Skip.into()
    }
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
                        C.bg_green_lighter,
                        C.hover__bg_green_light,
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
                        C.bg_blue_lighter,
                        C.hover__bg_blue_light,
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
                C.pin_x,
                C.pin_b,
                C.text_center,
                C.font_bold,
                C.text_blue_lighter
            ],
            "Press any key and look at the console log"
        ],
        // time
        div![
            class![
                C.mb_6,
                C.absolute,
                C.pin_x,
                C.pin_b,
                C.text_center,
                C.font_bold,
                C.text_xl,
                C.text_green_light
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
