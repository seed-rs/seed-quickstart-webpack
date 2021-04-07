use crate::{
    generated::css_classes::C, image_src, Msg, Urls, MAIL_TO_HELLWEB,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view(base_url: &Url) -> Node<Msg> {
    div![
        C![
            C.flex_grow,
        ],
        // Main section
        section![
            C![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                // sm__
                C.sm__h_890px,
                // lg__
                C.lg__h_1420px,
            ],
            // Left background
            div![
                C![
                    C.absolute,
                    C.left_0,
                    C.inset_y_0,
                    C.w_1of2,
                    C.bg_yellow_4,
                ]
            ],
            div![
                C![
                    C.relative,
                    C.flex,
                    C.justify_center,
                ],
                // Martin Kavík container
                div![
                    C![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        // sm__
                        C.sm__h_550px,
                        // lg__
                        C.lg__h_860px,
                        C.lg__rounded_bl_260px,
                    ],
                    // Martin Kavík
                    div![
                        C![
                            C.mt_40,
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            // sm__
                            C.sm__mt_64,
                            C.sm__ml_20,
                            C.sm__w_md,
                            // lg__
                            C.lg__mt_72,
                            C.lg__ml_20,
                            C.lg__w_216,
                        ],
                        h1![
                            C![
                                C.inline,
                                C.leading_tight,
                                C.text_31,
                                C.text_gray_10
                                // sm__
                                C.sm__text_40,
                                // lg__
                                C.lg__leading_none,
                                C.lg__text_120,
                            ],
                            span![
                                "Martin "
                            ],
                            span![
                                C![
                                    C.font_bold
                                ],
                                "Kavík"
                            ],
                        ],
                        span![
                            C![
                                C.text_21,
                                C.text_gray_7,
                                // sm__
                                C.sm__text_32,
                                // lg__
                                C.lg__text_60,
                            ],
                            "\u{00A0}is",
                            br![],
                            "a developer",
                            br![],
                            "with 7+ years of experience",
                            br![],
                            "who likes to design and ..."
                        ]
                    ],
                ],
            ],
            // Gear
            img![
                C![
                    C.absolute
                    C.top_0,
                    C.left_full,
                    C._ml_40,
                    C._mt_56,
                    C.w_md,
                    C.max_w_none,
                    C.blur,
                    // sm__
                    C.sm___mt_64,
                    C.sm__w_750px,
                    // lg__
                    C.lg__w_900px,
                ],
                attrs!{
                    At::Src => image_src("gear.svg")
                }
            ],
        ],
        // Seed section
        section![
            C![
                C.relative,
                C._mt_48,
                C.pt_px,
                C.rounded_tl_140px,
                C.bg_blue_8,
                // lg__
                C.lg___mt_80,
                C.lg__rounded_tl_330px,
            ],
            // Main list
            div![
                C![
                    C._ml_5,
                    C._mt_48,
                    C.max_w_lg,
                    C.flex,
                    C.justify_end,
                    C.relative,
                    // sm__
                    C.max_w_4xl,
                    // lg__
                    C.lg___mt_92,
                    C.lg__max_w_8xl,
                    C.lg__mx_auto,
                ],
                // Right background
                div![
                    C![
                        C.absolute,
                        C.right_0,
                        C.inset_y_0,
                        C._right_50vw,
                        C.w_50vw,
                        C.bg_gray_1
                    ]
                ],
                // List
                div![
                    C![
                        C.relative,
                        C.pl_4,
                        C.rounded_bl_45px,
                        C.font_display,
                        C.text_17,
                        C.text_gray_8,
                        C.bg_gray_1,
                        C.overflow_hidden,
                        // sm__
                        C.sm__pl_8,
                        C.sm__pr_2,
                        C.sm__text_26,
                        // lg__
                        C.lg__pl_16,
                        C.lg__text_45,
                        C.lg__rounded_bl_140px,
                    ],
                    ul![
                        C![
                            C.w_xs,
                            C.pl_2,
                            C.py_8,
                            // sm__
                            C.sm__w_120,
                            // lg__
                            C.lg__w_204,
                            C.lg__pt_32,
                            C.lg__pb_24,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    // https://stackoverflow.com/a/39900080
                                    "▶\u{fe0e}"
                                ],
                                "To work on your project."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Readable code and UI."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Rust, Affinity Designer and Figma."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "Receiving mails. ",
                                    a![
                                        attrs!{
                                            At::Href => MAIL_TO_KAVIK
                                        },
                                        C![
                                            C.underline,
                                            C.underline_yellow_7,
                                            C.font_semibold
                                        ],
                                        "martin@kavik.cz"
                                    ]
                                ]
                            ]
                        ],
                    ]
                ]
            ],
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_20,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_48,
                        C.sm__w_md,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_236,
                    ],
                    // Github projects
                    h2![
                        C![
                            C.font_display,
                            C.text_23,
                            C.text_blue_3,
                            C.text_center,
                            // sm__
                            C.sm__text_50,
                            // lg__
                            C.lg__text_80,
                        ],
                        span![
                            C![
                                C.font_thin
                            ],
                            "TOP-5"
                        ],
                        span![
                            C![
                                C.font_normal
                            ],
                            " GITHUB PROJECTS"
                        ]
                    ],
                    // Testimonial 1
                    div![
                        C![
                            C.mt_20,
                            C.text_right,
                            // sm__
                            C.sm__mt_40,
                            // lg__
                            C.lg__mt_64,
                        ],
                        div![
                            C![
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Awesome, awesome framework!"
                        ],
                        div![
                            C![
                                C.mt_2,
                                C.mr_2,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_4,
                                C.sm__text_20,
                                // lg__
                                C.lg__mt_6,
                                C.lg__mr_8,
                                C.lg__text_35,
                            ],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/issues/193#issue-479188076"
                                },
                                C![
                                    C.underline,
                                    C.underline_blue_5,
                                ],
                                "rebo"
                            ]
                        ]
                    ],
                    // Testimonial 2
                    div![
                        C![
                            C.mt_8,
                            C.text_right,
                            // sm__
                            C.sm__mt_12,
                            // lg__
                            C.lg__mt_16,
                        ],
                        div![
                            C![
                                C.mr_4,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4,
                                // lg__
                                C.lg__mr_16,
                            ],
                            "Seed rocks, and ",
                            br![],
                            "Martin makes it better."
                        ],
                        div![
                            C![
                                C.mt_2,
                                C.mr_5,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_4,
                                C.sm__text_20,
                                // lg__
                                C.lg__mt_8,
                                C.lg__mr_20,
                                C.lg__text_35,
                            ],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/MartinKavik/seed-rs-realworld/issues/1#issuecomment-525413644"
                                },
                                C![
                                    C.underline,
                                    C.underline_blue_5,
                                ],
                                "robwebbjr"
                            ]
                        ]
                    ],
                    // Seed
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                        },
                        C![
                            C.block,
                            C.relative,
                            C.mt_8,
                            C.pt_5,
                            C.pb_3,
                            C.w_36,
                            C.rounded_tr_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_24,
                            C.sm__pt_8,
                            C.sm__pb_8,
                            C.sm__w_64,
                            C.sm__rounded_tr_55px,
                            // lg__
                            C.lg__mt_32,
                            C.lg__pt_12,
                            C.lg__pb_12,
                            C.lg__w_md,
                            C.lg__rounded_tr_90px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            C![
                                C.h_18,
                                C.max_w_none,
                                // sm__
                                C.sm__h_32
                                // lg__
                                C.lg__ml_32,
                                C.lg__h_40,
                            ],
                            attrs!{
                                At::Src => image_src("seed_logo.svg")
                            }
                        ]
                    ],
                    ul![
                        C![
                            C.mt_10,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_20,
                            // lg__
                            C.lg__mt_32,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "Seed"
                                        ],
                                    ],
                                    "\u{00A0}is an open-source Rust framework for creating fast and reliable web apps running in WebAssembly."
                                ]
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                                // lg__
                                C.lg__mt_16,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I'm the main contributor."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                                // lg__
                                C.lg__mt_16,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I've designed the logo."
                            ]
                        ],
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                        },
                        C![
                            C.block,
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_16,
                            C.sm__mb_48,
                            // lg__
                            C.lg__mt_24,
                            C.lg__mb_64,
                        ],
                        span![
                            C![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_blue_2
                            ],
                            "awesome-seed-rs"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ]
                ]
            ]
        ],
        // RealWorld section
        section![
            C![
                C.bg_blue_10
            ],
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_236,
                    ],
                    // Testimonial
                    div![
                        C![
                            C.mt_10,
                            C.mr_2,
                            C.text_right,
                        ],
                        div![
                            C![
                                C.mr_3,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Your real world example really is the mother of all examples."
                        ],
                        div![
                            C![
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__mt_6,
                                C.sm__text_20,
                                // lg__
                                C.lg__mt_8,
                                C.lg__text_35,
                            ],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/pull/189#issuecomment-516095587"
                                },
                                C![
                                    C.underline,
                                    C.underline_blue_5,
                                ],
                                "theduke"
                            ]
                        ]
                    ],
                    // RealWorld example app
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-rs-realworld"
                        },
                        C![
                            C.block,
                            C.relative,
                            C.mt_12,
                            C.ml_1,
                            C.pt_8,
                            C.pb_6,
                            C.rounded_tl_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_32,
                            C.sm__pt_12,
                            C.sm__pb_10,
                            C.sm__rounded_tl_55px,
                            // lg__
                            C.lg__mt_40,
                            C.lg__ml_32,
                            C.sm__pt_16,
                            C.sm__pb_12,
                            C.lg__rounded_tl_90px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C._right_50vw,
                                C.inset_y_0,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            C![
                                C.h_12,
                                C.max_w_none,
                                // sm__
                                C.sm__h_20
                                // lg__
                                C.lg__h_32,
                            ],
                            attrs!{
                                At::Src => image_src("realworld_logo.png")
                            }
                        ]
                    ],
                    ul![
                        C![
                            C.mt_12,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                            C.sm__ml_8,
                            // lg__
                            C.lg__mt_40,
                            C.lg__ml_0,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/seed-rs-realworld"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "RealWorld example"
                                        ],
                                    ],
                                    "\u{00A0}is a Seed codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the RealWorld spec and API."
                                ]
                            ]
                        ],
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-rs-realworld"
                        },
                        C![
                            C.block,
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_48,
                            // lg__
                            C.lg__mt_32,
                            C.lg__mb_64,
                        ],
                        span![
                            C![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_blue_2
                            ],
                            "seed-rs-realworld"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Kavik.cz section
        section![
            C![
                C.bg_blue_6
            ],
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_md,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_204,
                    ],
                    // Testimonial
                    div![
                        C![
                            C.mt_10,
                            C.mr_3,
                            C.text_right,
                        ],
                        div![
                            C![
                                C.mr_4,
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "Fork it, use it!"
                        ],
                        div![
                            C![
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__text_20,
                                // lg__
                                C.lg__mt_8,
                                C.lg__text_35,
                            ],
                            "- me"
                        ]
                    ],
                    // MK
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/kavik.cz"
                        },
                        C![
                            C.block,
                            C.relative,
                            C.mt_8,
                            C.pt_4,
                            C.pb_2,
                            C.w_36,
                            C.rounded_tr_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_20,
                            C.sm__w_56,
                            C.sm__pt_10,
                            C.sm__pb_8,
                            C.sm__rounded_tr_55px,
                            // lg__
                            C.lg__w_96,
                            C.lg__rounded_tr_90px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            C![
                                C.ml_6,
                                C.h_20,
                                C.max_w_none,
                                // sm__
                                C.sm__h_32,
                                // lg__
                                C.lg__ml_32,
                                C.lg__h_40,
                            ],
                            attrs!{
                                At::Src => image_src("logo.svg")
                            }
                        ]
                    ],
                    ul![
                        C![
                            C.mt_10,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                            // lg__
                            C.lg__mt_32,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/kavik.cz"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "kavik.cz"
                                        ],
                                    ],
                                    "\u{00A0}is this website."
                                ]
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                                // lg__
                                C.lg__mt_16,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "You can fork it, modify it and use it as your own website."
                            ]
                        ],
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/kavik.cz"
                        },
                        C![
                            C.block,
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_56,
                            // lg__
                            C.lg__mt_32,
                            C.lg__mb_64,
                        ],
                        span![
                            C![
                                C.text_blue_3
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_blue_2
                            ],
                            "kavik.cz"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Seed Quickstart section
        section![
            C![
                C.rounded_br_140px,
                C.bg_blue_10,
                // lg__
                C.lg__rounded_br_330px,
            ],
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_216,
                    ],
                    // Testimonial
                    div![
                        C![
                            C.mt_10,
                            C.ml_5,
                            // sm__
                            C.sm__ml_12,
                        ],
                        div![
                            C![
                                C.font_display,
                                C.italic,
                                C.text_yellow_4
                            ],
                            "It's great!"
                        ],
                        div![
                            C![
                                C.ml_12,
                                C.mt_4,
                                C.font_display,
                                C.text_15,
                                C.text_blue_3,
                                // sm__
                                C.sm__ml_20,
                                C.sm__text_20,
                                // lg__
                                C.lg__mt_8,
                                C.lg__ml_40,
                                C.lg__text_35,
                            ],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/issues/200#issuecomment-522247909"
                                },
                                C![
                                    C.underline,
                                    C.underline_blue_5,
                                ],
                                "rebo"
                            ]
                        ]
                    ],
                    // Seed Quickstart
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-quickstart-webpack"
                        },
                        C![
                            C.block,
                            C.relative,
                            C.mt_12,
                            C.ml_6,
                            C.pt_8,
                            C.pb_6,
                            C.rounded_tl_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_32,
                            C.sm__pt_12,
                            C.sm__pb_10,
                            C.sm__rounded_tl_55px,
                            // lg__
                            C.lg__ml_48,
                            C.lg__pt_16,
                            C.lg__pb_12,
                            C.lg__rounded_tl_90px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C._right_50vw,
                                C.inset_y_0,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        div![
                            C![
                                C.ml_2,
                                C.font_display,
                                C.font_bold,
                                C.text_34,
                                // sm__
                                C.sm__ml_4,
                                C.sm__text_60,
                                // lg__
                                C.lg__ml_6,
                                C.lg__text_80,
                            ],
                            "Seed Quickstart"
                        ],
                    ],
                    ul![
                        C![
                            C.mt_12,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_24,
                            // lg__
                            C.lg__mt_32,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/seed-quickstart-webpack"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "Seed Quickstart"
                                        ],
                                    ],
                                    "\u{00A0}is a template for web apps with Seed, TailwindCSS, Typescript and Webpack."
                                ]
                            ]
                        ],
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-quickstart-webpack"
                        },
                        C![
                            C.block,
                            C.mt_10,
                            C.mb_56,
                            C.text_right,
                            C.font_display,
                            C.text_15,
                            // sm__
                            C.sm__mt_20,
                            C.sm__mb_96,
                            C.sm__text_26,
                            // lg__
                            C.lg__mt_32,
                            C.lg__mb_132,
                            C.lg__text_45,
                        ],
                        span![
                            C![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_blue_2
                            ],
                            "seed-quickstart-webpack"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Hellweb section
        section![
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_16,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_40,
                        C.sm__w_132,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_236,
                    ],
                    // Hellweb container
                    div![
                        C![
                            C.relative,
                            C._mt_40,
                            C.w_xs,
                            C.py_12,
                            C.rounded_tr_140px,
                            C.bg_gray_1,
                            // sm__
                            C.sm__w_132,
                            C.sm___mt_80,
                            // lg__
                            C.lg__w_204,
                            C.lg___mt_120,
                            C.lg__rounded_tr_330px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_gray_1,
                            ]
                        ],
                        // Hellweb
                        a![
                            attrs!{
                                At::Href => "https://github.com/MartinKavik/hellweb-pain"
                            },
                            img![
                                C![
                                    C.mt_1,
                                    C.ml_8,
                                    C.h_12,
                                    // sm__
                                    C.sm__mt_20,
                                    C.sm__h_20,
                                    // lg__
                                    C.lg__mt_24,
                                    C.lg__ml_16,
                                    C.lg__h_32,
                                ],
                                attrs!{
                                    At::Src => image_src("hellweb_logo.svg")
                                }
                            ],
                        ]
                    ],
                    ul![
                        C![
                            C.text_gray_10,
                            // sm__
                            C.sm__mt_16,
                            // lg__
                            C.sm__mt_24,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/hellweb-pain"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "Hellweb"
                                        ],
                                    ],
                                    "\u{00A0}will be a collection of Rust libraries and applications which solve your pain points and explore new ideas."
                                ]
                            ]
                        ],
                        li![
                            C![
                                C.mt_8,
                                // sm__
                                C.sm__mt_16,
                                // lg__
                                C.lg__mt_32,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_nowrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "What do you hate or what annoys you about ",
                                    span![
                                        C![
                                            C.font_bold,
                                        ],
                                        "web design & development"
                                    ],
                                    "? Don't hesitate to create an issue or contact me - ",
                                    a![
                                        attrs!{
                                            At::Href => MAIL_TO_HELLWEB
                                        },
                                        C![
                                            C.underline,
                                            C.underline_yellow_7,
                                        ],
                                        "martin@hellweb.app"
                                    ]
                                ]
                            ]
                        ]
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/hellweb-pain"
                        },
                        C![
                            C.block
                            C.mt_10,
                            C.mr_2,
                            C.text_right,
                            // sm__
                            C.sm__mt_20,
                            // lg__
                            C.lg__mt_32,
                            C.lg__mr_16,
                        ],
                        span![
                            C![
                                C.text_gray_5
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_gray_9
                            ],
                            "hellweb-pain"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ],
                    // About your new developer
                    a![
                        attrs!{
                            At::Href => Urls::new(base_url).about()
                        },
                        C![
                            C.block,
                            C.mt_20,
                            C.mb_16,
                            C.ml_5,
                            C.flex,
                            C.items_center,
                            C.justify_center,
                            C.text_19,
                            C.text_gray_10,
                            C.hover__text_yellow_7,
                            // sm__
                            C.sm__mt_40,
                            C.sm__mb_24,
                            C.sm__text_28,
                            // lg__
                            C.lg__mt_64,
                            C.lg__mb_48,
                            C.lg__text_50,
                        ],
                        ev(Ev::Click, |_| Msg::ScrollToTop),
                        span![
                            C![
                                C.font_semibold
                            ],
                            "About"
                        ],
                        "\u{00A0}your new developer",
                        img![
                            C![
                                C.mt_1,
                                C.h_12,
                                // sm__
                                C.sm__h_16,
                                // lg__
                                C.lg__h_32,
                            ],
                            attrs!{
                                At::Src => image_src("next.svg")
                            }
                        ],
                    ]
                ]
            ]
        ],
        // Circles
        div![
            C![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_310px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_1290px,
                C.lg__ml_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_790px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_1310px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_2840px,
                C.lg__mr_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_1760px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3040px,
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_5030px,
                C.lg__ml_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_2340px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3870px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_6070px,
                C.lg__mr_108,
            ]
        ],
    ]
}
