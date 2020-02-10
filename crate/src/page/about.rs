use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK, Page,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view() -> impl View<Msg> {
    div!["about",
        a![
            class![
                C.block,
                C.text_blue_6,
            ],
            attrs!{
                At::Href => Page::Home.to_href(),
            },
            "Go to Home"
        ]
    ]
}
