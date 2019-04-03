use seed::*;
use seed::prelude::*;
use crate::ts_apis;

pub type CustomEventDetail = JsValue;


// Model

pub struct Model {
    pub clicks: i32,
    pub random_number: i32,
    pub clock_time: Option<String>
}

impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            random_number: ts_apis::helpers::get_random_number(0,100),
            clock_time: None
        }
    }
}


// Update

#[derive(Clone)]
pub enum Msg {
    Increment,
    NewRandomNumber,
    ClockTick(CustomEventDetail)
}

pub fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0,100),
        Msg::ClockTick(detail) => model.clock_time = detail.as_string()
    }
    Render.into()
}


// View

pub fn view(model: &Model) -> El<Msg> {
    div![
        class!["bg-custom"],
        div![ 
            class!["h-screen", "w-screen", "flex", "flex-wrap", "justify-center", "content-center"],
            img![                      
                attrs!{At::Class => "mb-8"; At::Src => "static/images/quickstart.png";}
            ],
            div![
                class!["flex", "flex-wrap", "justify-center", "content-center"],
                button![ 
                    class!["mb-8", "mx-8", "p-4", "rounded", "shadow-md", "bg-green-lighter", "hover:bg-green-light", "font-bold"],
                    simple_ev(Ev::Click, Msg::Increment), 
                    format!("Clicks: {}", model.clicks) 
                ],
                button![ 
                    class!["mb-8", "p-4", "rounded", "shadow-md", "bg-blue-lighter", "hover:bg-blue-light", "font-bold"],
                    simple_ev(Ev::Click, Msg::NewRandomNumber), 
                    format!("Random number from Typescript: {}", model.random_number) 
                ]
            ]
        ],
        div![
            class!["mb-6", "absolute", "pin-x", "pin-b", "text-center", "font-bold", "text-xl", "text-green-light"],
            format!("{}", if let Some(ref time) = model.clock_time { time } else { "" } )
        ]
    ]
}

// Subscriptions

// https://seed-rs.org/guide/3
// > There's currently a bug where window listeners won't work until the first (non-window-listener) update is triggered.
pub fn window_events(_model: &Model) -> Vec<dom_types::Listener<Msg>> {
    // @TODO log keyboard to console?
    Vec::new()
}

pub fn custom_events(_model: &Model) -> Vec<Msg> {
    Vec::new()
}