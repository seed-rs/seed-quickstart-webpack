#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use crate::css_classes::C;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Msg>() -> Node<Msg> {
    section![
        C![
            C.fade_in,
            C.hero,
            C.is_bold,
            C.is_fullheight_with_navbar
        ],
        div![
            C![C.hero_body],
            div![
                C![C.container, C.has_text_centered],
                h1![
                    C![C.title C.is_1],
                   "404 Page Not Found"
                ],
                h2!["An unexpected error has occurred. Please contact the site owner.", C![C.subtitle]],
            ]
        ],
    ]
}
