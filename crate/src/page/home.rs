use crate::{generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB, MAIL_TO_KAVIK, Model};
use seed::{prelude::*, *};
use counter_component_interface::{Id as CounterId, Message as CounterMessage};

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> impl View<Msg> {
    div!["home",
        a![
            class![
                C.block,
                C.text_blue_6,
            ],
            attrs!{
                At::Href => Page::About.to_href(),
            },
            "Go to About"
        ],
        counter(model.counter_a),
        counter(model.counter_b),
    ]
}

fn counter(id: CounterId) -> Node<Msg> {
    div![
        button![
            class![C.bg_blue_4, C.p_2, C.m_2, C.rounded,],
            "Increment",
            ev(Ev::Click, move |_| {
                update_counter(id, CounterMessage::Increment);
                Msg::NoOp
            }),
        ],
        view_counter(id),
        button![
            class![C.bg_blue_4, C.p_2, C.m_2, C.rounded,],
            "Decrement",
            ev(Ev::Click, move |_| {
                update_counter(id, CounterMessage::Decrement);
                Msg::NoOp
            }),
        ],
    ]
}

// ------ ------
//    Extern
// ------ ------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = counter, js_name = update)]
    fn update_counter(id: CounterId, message: CounterMessage);
    #[wasm_bindgen(js_namespace = counter, js_name = view)]
    fn view_counter(id: CounterId) -> String;
}

