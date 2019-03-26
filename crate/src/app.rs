use seed::*;
use seed::prelude::*;

// Model

pub struct Model {
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
pub enum Msg {
    Increment,
}

pub fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.val += 1,
    }
    Render.into()
}


// View

pub fn view(model: &Model) -> El<Msg> {
    div![   
        button![ 
            class!["shadow-lg", "text-green", "p-4"],
            simple_ev(Ev::Click, Msg::Increment), 
            format!("Clicks: {}", model.val) 
        ],
        button![ 
            class!["shadow-lg", "text-green", "p-4"],
            simple_ev(Ev::Click, Msg::Increment), 
            format!("Random number from Typescript: {}", 0) 
        ]
    ]
}