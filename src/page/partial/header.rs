use crate::{
    asset_path,
    css_classes::C,
    image_src, Model, Msg, Page, ScrollHistory, Urls,
    Visibility::{self, *},
};
use seed::{prelude::*, *};

fn header_visibility(
    menu_visibility: Visibility,
    scroll_history: &ScrollHistory,
) -> Visibility {
    let menu_is_visible = menu_visibility == Visible;
    // You can go higher on the mobile phones.
    let at_the_top_or_higher = *scroll_history.back().unwrap_or(&0) <= 0;
    let scrolling_up = scroll_history.front() >= scroll_history.back();

    if menu_is_visible || at_the_top_or_higher || scrolling_up {
        return Visible;
    }
    Hidden
}

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let show_header =
        header_visibility(model.menu_visibility, &model.scroll_history)
            == Visible;
    nodes![
        // Header background and line container
        IF!(show_header =>
            div![
                C![],
                // Header background
                div![C![]],
                // Bottom header line
                div![
                    C![],
                    div![C![],],
                    div![C![],]
                ],
            ]
        ),
        // Photo 1
        IF!(model.page == Page::About =>
            div![
                C![],
                img![
                    C![],
                    attrs! {
                        At::Src => image_src("photo_1.png")
                    }
                ],
            ]
        ),
        // Menu
        IF!(model.menu_visibility == Visible =>
            div![
                C![],
                div![
                    C![],
                    ul![
                        C![],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).home()
                                },
                                ev(Ev::Click, |_| Msg::ScrollToTop),
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "Home & Projects"
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).about()
                                },
                                ev(Ev::Click, |_| Msg::ScrollToTop),
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "About"
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => asset_path("Martin_Kavik_resume.pdf")
                                },
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "Resume",
                                span![C![], ".pdf"]
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => "https://github.com/MartinKavik"
                                },
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "GitHub",
                                img![
                                    C![],
                                    attrs! {
                                        At::Src => image_src("link_arrow.svg")
                                    }
                                ]
                            ]
                        ],
                    ],
                ]
            ]
        ),
        // Header
        IF!(show_header =>
            header![
                C![],
                // Header controls container
                div![
                    C![],
                    // Logo
                    a![
                        attrs! {
                            At::Href => Urls::new(&model.base_url).home()
                        },
                        ev(Ev::Click, |_| Msg::ScrollToTop),
                        ev(Ev::Click, |_| Msg::HideMenu),
                        img![
                            C![],
                            attrs! {
                                At::Src => image_src("logo.svg")
                            }
                        ],
                    ],
                    // Links
                    ul![
                        C![],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).home()
                                },
                                ev(Ev::Click, |_| Msg::ScrollToTop),
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "Home & Projects"
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => Urls::new(&model.base_url).about()
                                },
                                ev(Ev::Click, |_| Msg::ScrollToTop),
                                ev(Ev::Click, |_| Msg::HideMenu),
                                "About"
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => asset_path("Martin_Kavik_resume.pdf")
                                },
                                "Resume",
                                span![
                                    C![],
                                    ".pdf"
                                ]
                            ]
                        ],
                        li![
                            C![],
                            a![
                                C![],
                                attrs! {
                                    At::Href => "https://github.com/MartinKavik"
                                },
                                "GitHub",
                                img![
                                    C![],
                                    attrs! {
                                        At::Src => image_src("link_arrow.svg")
                                    }
                                ]
                            ]
                        ],
                    ],
                    // Hamburger
                    div![
                        C![],
                        ev(Ev::Click, |_| Msg::ToggleMenu),
                        img![
                            id!("hamburger"),
                            C![],
                            if model.in_prerendering {
                                attrs! {
                                    At::Src => image_src("loading.svg")
                                }
                            } else {
                                attrs! {
                                    At::Src => if model.menu_visibility == Visible {
                                        image_src("cross.svg")
                                    } else {
                                        image_src("hamburger.svg")
                                    }
                                }
                            }
                        ]
                    ],
                    // Spacer
                    div![C![],],
                ],
                // Bottom header line
                div![
                    C![],
                    div![C![],],
                    div![C![],]
                ],
            ]
        ),
    ]
}
