use crate::{css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![],
        h1![
            C![],
            "PAGE NOT FOUND!"
        ],
        // Sad mouth
        svg![
            C![],
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
