use crate::generated::css_classes::C;
use crate::ts_apis;
use seed::{events::Listener, prelude::*};
use seed::*;
use serde::Deserialize;

// ------ ------
//     Model
// ------ ------

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

// ------ ------
//     Init
// ------ ------

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![
        keyboard_ev("keydown", |ev| Msg::KeyPressed(ev.key())),
        // `trigger_update_handler` processes JS event
        // and forwards it to `update` function.
        trigger_update_handler()
    ]
}

// ------ ------
//    Update
// ------ ------

// We trigger `OnClockTick` only from JS land
#[allow(dead_code)]
// `Deserialize` is required for receiving messages from JS land
// (see `trigger_update_handler`)
#[derive(Clone, Deserialize)]
pub enum Msg {
    Increment,
    NewRandomNumber,
    KeyPressed(String),
    OnClockTick(String)
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0, 100),
        Msg::KeyPressed(key) => {
            log!(key);
            orders.skip();
        }
        Msg::OnClockTick(time) => {
            model.clock_time = Some(time);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
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
