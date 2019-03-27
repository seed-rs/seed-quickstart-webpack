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
        class!["h-screen", "w-screen", "flex", "flex-wrap", "justify-center", "content-center"],
        button![ 
            class!["mt-8", "mr-8", "p-4", "rounded", "shadow-md", "bg-green-lighter", "hover:bg-green-light"],
            simple_ev(Ev::Click, Msg::Increment), 
            format!("Clicks: {}", model.val) 
        ],
        button![ 
            class!["mt-8", "p-4", "rounded", "shadow-md", "bg-blue-lighter", "hover:bg-blue-light"],
            simple_ev(Ev::Click, Msg::Increment), 
            format!("Random number from Typescript: {}", 0) 
        ]
    ]
}