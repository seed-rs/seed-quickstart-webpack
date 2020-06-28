use crate::{css_classes::C, image_src, Msg, MAIL_TO_KAVIK};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    footer![
        C![],
        div![
            C![],
            div![
                C![],
                img![
                    C![],
                    attrs! {
                        At::Src => image_src("logo.svg")
                    }
                ],
                span![
                    C![],
                    "2019"
                ]
            ],
            a![
                attrs! {
                    At::Href => MAIL_TO_KAVIK,
                },
                C![],
                "martin@kavik.cz"
            ],
            div![
                C![],
                ev(Ev::Click, |_| Msg::ScrollToTop),
                img![
                    C![],
                    attrs! {
                        At::Src => image_src("top.svg")
                    }
                ],
            ]
        ]
    ]
}
