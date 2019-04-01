use seed::*;
use seed::prelude::*;
use crate::ts_apis;

// Model

pub struct Model {
    pub clicks: i32,
    pub random_number: i32
}

impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            random_number: ts_apis::helpers::get_random_number(0,100)
        }
    }
}


// Update

#[derive(Clone)]
pub enum Msg {
    Increment,
    NewRandomNumber
}

pub fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0,100),
    }
    Render.into()
}


// View

pub fn view(model: &Model) -> El<Msg> {
    div![ 
        class!["h-screen", "w-screen", "flex", "flex-wrap", "justify-center", "content-center", "bg-custom"],
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
    ]
}