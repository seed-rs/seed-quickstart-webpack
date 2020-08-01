#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use crate::css_classes::C;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

#[derive(Copy, Clone)]
pub struct DoReset;

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        value: 0,
        _sub_handle: orders.subscribe_with_handle(|_: DoReset| Msg::Reset),
    }
}

// ------ ------
//     Model
// ------ ------
pub struct Model {
    pub value: i32,
    _sub_handle: SubHandle,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn counter_url(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Increment => model.value += 1,
        Msg::Decrement => model.value -= 1,
        Msg::Reset => model.value = 0,
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![C.fade_in, C.pl_4, C.pt_4, C.field, C.is_grouped],
        p![
            C![C.control],
            button![
                C![C.button, C.is_dark],
                ev(Ev::Click, |_| Msg::Decrement),
                span![C![C.icon], i![C![C.fas, C.fa_minus]]]
            ]
        ],
        p![
            C![C.control],
            p![C![C.is_size_3, C.has_text_centered], model.value]
        ],
        p![
            C![C.control],
            button![
                C![C.button, C.is_dark],
                ev(Ev::Click, |_| Msg::Increment),
                span![C![C.icon], i![C![C.fas, C.fa_plus]]]
            ]
        ],
    ]
}
