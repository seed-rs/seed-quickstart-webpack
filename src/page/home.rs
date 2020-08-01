use crate::{
    css_classes::C, image_src, Msg, Urls, MAIL_TO_HELLWEB,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view(base_url: &Url) -> Node<Msg> {
    div![
        C![],
        // Main section
        section![
            C![],
            // Left background
            div![
                C![]
            ],
            div![
                C![],
                // Martin Kavík container
                div![
                    C![],
                    // Martin Kavík
                    div![
                        span![
                            i![C![C.fa, C.fa_user]],
                        ],
                                               span![
                            i![C![C.far, C.fa_user]],
                        ],
                        C![],
                        h1![
                            C![],
                            span![
                                "Martin "
                            ],
                            span![
                                C![],
                                "Kavík"
                            ],
                        ],
                        span![
                            C![],
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
                C![],
                attrs!{
                    At::Src => image_src("gear.svg")
                }
            ],
        ],
        // Seed section
        section![
            C![],
            // Main list
            div![
                C![],
                // Right background
                div![
                    C![]
                ],
                // List
                div![
                    C![],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    // https://stackoverflow.com/a/39900080
                                    "▶\u{fe0e}"
                                ],
                                "To work on your project."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                "Readable code and UI."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                "Rust, Affinity Designer and Figma."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "Receiving mails. ",
                                    a![
                                        attrs!{
                                            At::Href => MAIL_TO_KAVIK
                                        },
                                        C![],
                                        "martin@kavik.cz"
                                    ]
                                ]
                            ]
                        ],
                    ]
                ]
            ],
            div![
                C![],
                // Section content container
                div![
                    C![],
                    // Github projects
                    h2![
                        C![],
                        span![
                            C![],
                            "TOP-5"
                        ],
                        span![
                            C![],
                            " GITHUB PROJECTS"
                        ]
                    ],
                    // Testimonial 1
                    div![
                        C![],
                        div![
                            C![],
                            "Awesome, awesome framework!"
                        ],
                        div![
                            C![],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/issues/193#issue-479188076"
                                },
                                C![],
                                "rebo"
                            ]
                        ]
                    ],
                    // Testimonial 2
                    div![
                        C![],
                        div![
                            C![],
                            "Seed rocks, and ",
                            br![],
                            "Martin makes it better."
                        ],
                        div![
                            C![],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/MartinKavik/seed-rs-realworld/issues/1#issuecomment-525413644"
                                },
                                C![],
                                "robwebbjr"
                            ]
                        ]
                    ],
                    // Seed
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                        },
                        C![],
                        // Extended background
                        div![
                            C![]
                        ],
                        // Logo
                        img![
                            C![],
                            attrs!{
                                At::Src => image_src("seed_logo.svg")
                            }
                        ]
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                                        },
                                        h3![
                                            C![],
                                            "Seed"
                                        ],
                                    ],
                                    "\u{00A0}is an open-source Rust framework for creating fast and reliable web apps running in WebAssembly."
                                ]
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                "I'm the main contributor."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
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
                        C![],
                        span![
                            C![],
                            "MartinKavik/"
                        ],
                        span![
                            C![],
                            "awesome-seed-rs"
                        ],
                        img![
                            C![],
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
            C![],
            div![
                C![],
                // Section content container
                div![
                    C![],
                    // Testimonial
                    div![
                        C![],
                        div![
                            C![],
                            "Your real world example really is the mother of all examples."
                        ],
                        div![
                            C![],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/pull/189#issuecomment-516095587"
                                },
                                C![],
                                "theduke"
                            ]
                        ]
                    ],
                    // RealWorld example app
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-rs-realworld"
                        },
                        C![],
                        // Extended background
                        div![
                            C![]
                        ],
                        // Logo
                        img![
                            C![],
                            attrs!{
                                At::Src => image_src("realworld_logo.png")
                            }
                        ]
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/seed-rs-realworld"
                                        },
                                        h3![
                                            C![],
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
                        C![],
                        span![
                            C![],
                            "MartinKavik/"
                        ],
                        span![
                            C![],
                            "seed-rs-realworld"
                        ],
                        img![
                            C![],
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
            C![],
            div![
                C![],
                // Section content container
                div![
                    C![],
                    // Testimonial
                    div![
                        C![],
                        div![
                            C![],
                            "Fork it, use it!"
                        ],
                        div![
                            C![],
                            "- me"
                        ]
                    ],
                    // MK
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/kavik.cz"
                        },
                        C![],
                        // Extended background
                        div![
                            C![]
                        ],
                        // Logo
                        img![
                            C![],
                            attrs!{
                                At::Src => image_src("logo.svg")
                            }
                        ]
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/kavik.cz"
                                        },
                                        h3![
                                            C![],
                                            "kavik.cz"
                                        ],
                                    ],
                                    "\u{00A0}is this website."
                                ]
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
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
                        C![],
                        span![
                            C![],
                            "MartinKavik/"
                        ],
                        span![
                            C![],
                            "kavik.cz"
                        ],
                        img![
                            C![],
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
            C![],
            div![
                C![],
                // Section content container
                div![
                    C![],
                    // Testimonial
                    div![
                        C![],
                        div![
                            C![],
                            "It's great!"
                        ],
                        div![
                            C![],
                            "- ",
                            a![
                                attrs!{
                                    At::Href => "https://github.com/David-OConnor/seed/issues/200#issuecomment-522247909"
                                },
                                C![],
                                "rebo"
                            ]
                        ]
                    ],
                    // Seed Quickstart
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/seed-quickstart-webpack"
                        },
                        C![],
                        // Extended background
                        div![
                            C![]
                        ],
                        // Logo
                        div![
                            C![],
                            "Seed Quickstart"
                        ],
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/seed-quickstart-webpack"
                                        },
                                        h3![
                                            C![],
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
                        C![],
                        span![
                            C![],
                            "MartinKavik/"
                        ],
                        span![
                            C![],
                            "seed-quickstart-webpack"
                        ],
                        img![
                            C![],
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
                C![],
                // Section content container
                div![
                    C![],
                    // Hellweb container
                    div![
                        C![],
                        // Extended background
                        div![
                            C![]
                        ],
                        // Hellweb
                        a![
                            attrs!{
                                At::Href => "https://github.com/MartinKavik/hellweb-pain"
                            },
                            img![
                                C![],
                                attrs!{
                                    At::Src => image_src("hellweb_logo.svg")
                                }
                            ],
                        ]
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/hellweb-pain"
                                        },
                                        h3![
                                            C![],
                                            "Hellweb"
                                        ],
                                    ],
                                    "\u{00A0}will be a collection of Rust libraries and applications which solve your pain points and explore new ideas."
                                ]
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                div![
                                    C![],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "What do you hate or what annoys you about ",
                                    span![
                                        C![],
                                        "web design & development"
                                    ],
                                    "? Don't hesitate to create an issue or contact me - ",
                                    a![
                                        attrs!{
                                            At::Href => MAIL_TO_HELLWEB
                                        },
                                        C![],
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
                        C![],
                        span![
                            C![],
                            "MartinKavik/"
                        ],
                        span![
                            C![],
                            "hellweb-pain"
                        ],
                        img![
                            C![],
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
                        C![],
                        ev(Ev::Click, |_| Msg::ScrollToTop),
                        span![
                            C![],
                            "About"
                        ],
                        "\u{00A0}your new developer",
                        img![
                            C![],
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
            C![]
        ],
        div![
            C![]
        ],
        div![
            C![]
        ],
        div![
            C![]
        ],
    ]
}
