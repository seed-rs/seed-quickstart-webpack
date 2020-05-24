use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            C.mt_16,
            C.flex_grow,
            C.flex,
            C.flex_col,
            C.items_center,
            C.justify_center,
            // sm__
            C.sm__mt_24,
        ],
        h1![
            C![
                C.font_display,
                C.font_thin,
                C.text_29,
                C.text_gray_10,
                // sm__
                C.sm__text_45,
                // lg__
                C.lg__text_55,
            ],
            "PAGE NOT FOUND!"
        ],
        // Sad mouth
        svg![
            C![
                C.mt_5, C.w_16, // sm__
                C.sm__mt_6, C.sm__w_20, // lg__
                C.lg__mt_8, C.lg__w_24,
            ],
            style! {
                "background" => "rgba(0, 0, 0, 0) none repeat scroll 0% 0%",
                "transform" => "scaleY(-1)",
            },
            attrs! {
                At::ViewBox => "0 0 100 100",
                // @TODO: Rewrite once `preserveAspectRatio` is supported.
                At::Custom("preserveAspectRatio".into()) => "xMidYMid",
            },
            path![attrs! {
                // @TODO: Rewrite once `stroke` is supported.
                At::Custom("stroke".into()) => "none",
                At::D => "M10 50A40 40 0 0 0 90 50A40 42 0 0 1 10 50"
            }]
        ]
    ]
}
