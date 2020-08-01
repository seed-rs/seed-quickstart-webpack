use crate::{
    asset_path, css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view() -> Node<Msg> {
    div![
        C![],
        // Photo section
        section![
            C![],
            // Small photo background container
            div![
                C![],
                // Small photo background
                div![
                    C![]
                ],
            ],
            // Large photo background
            div![
                C![],
            ],
            // Gear
            img![
                C![],
                attrs!{
                    At::Src => image_src("gear.svg")
                }
            ],
        ],
        // Developer section
        section![
            C![],
            div![
                C![],
                // Right background
                div![
                    C![]
                ],
                // Extended background
                div![
                    C![]
                ],
                // I, developer
                h2![
                    C![],
                    "I, developer"
                ],
                ul![
                    C![],
                    li![
                        C![],
                        div![
                            C![],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "I was working as a ",
                            span![
                                C![],
                                "backend"
                            ],
                            " developer in a bank and for some startups and agencies last years."
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
                                "I'm also coming back to ",
                                span![
                                    C![],
                                    "frontend"
                                ],
                                " because it's finally possible to write reliable web apps. And I want to make more use of my ",
                                span![
                                    C![],
                                    "artistic"
                                ],
                                " self."
                            ]
                        ]
                    ],
                    li![
                        C![],
                        div![
                            C![],
                            "▶\u{fe0e}"
                        ],
                        div![
                            "People make mistakes.",
                            br![],
                            "That's why I setup linters, formatters and CI pipelines as ",
                            span![
                                C![],
                                "strict"
                            ],
                            " as possible and I want to write in ",
                            span![
                                C![],
                                "Rust"
                            ],
                            ".",
                            br![],
                            "Ideal tools \"bully\" me and don't have any configuration options."
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
                                "I often learn from ",
                                a![
                                    attrs!{
                                        At::Href => "https://www.packtpub.com/"
                                    },
                                    C![],
                                    "packtpub.com"
                                ],
                                ".",
                                br![],
                                "And I recommend to read book ",
                                a![
                                    attrs!{
                                        At::Href => "https://fsharpforfunandprofit.com/books/"
                                    },
                                    C![],
                                    "Domain Modeling Made Functional"
                                ],
                                "."
                            ]
                        ]
                    ],
                ]
            ]
        ],
        // Designer section
        section![
            C![],
            // Circles
            div![
                C![],
                div![
                    C![]
                ],
                div![
                    C![]
                ],
            ],
            // I, designer
            div![
                C![],
                h2![
                    C![],
                    "I, designer"
                ],
            ],
            ul![
                C![],
                li![
                    C![],
                    div![
                        C![],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I've designed logos, my resume and this website in ",
                        span![
                            C![],
                            "Affinity Designer"
                        ],
                        "."
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
                            "I'll use ",
                            span![
                                C![],
                                "Figma"
                            ],
                            " for my next website design."
                        ]
                    ]
                ],
                li![
                    C![],
                    div![
                        C![],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I have some experience with ",
                        span![
                            C![],
                            "Adobe XD"
                        ],
                        ", ",
                        span![
                            C![],
                            "Krita"
                        ],
                        " and ",
                        span![
                            C![],
                            "Rhino3D"
                        ],
                        "."
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
                            "I recommend to check ",
                            a![
                                attrs!{
                                    At::Href => "https://refactoringui.com/"
                                },
                                C![],
                                "refactoringui.com"
                            ],
                            ". I've bought their book and I use their ",
                            a![
                                attrs!{
                                    At::Href => "https://tailwindcss.com/"
                                },
                                C![],
                                "TailwindCSS"
                            ],
                            " in my projects."
                        ]
                    ]
                ],
            ]
        ],
        // Human section
        section![
            C![],
            // I, human
            h2![
                C![],
                "I, human"
            ],
            // Personal life
            div![
                C![],
                // Extended background
                div![
                    C![]
                ],
                // Content container
                div![
                    C![],
                    h3![
                        C![],
                        "Personal life"
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'm INTJ. When I'm not ",
                                span![
                                    C![],
                                    "creating"
                                ],
                                " something, I usually read or go to gym."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I like to spend my vacation at the cottage - hiking, cycling, driving, etc."
                            ]
                        ]
                    ]
                ]
            ],
            // Work life
            div![
                C![],
                // Extended background
                div![
                    C![]
                ],
                // Content container
                div![
                    C![],
                    h3![
                        C![],
                        "Work life"
                    ],
                    ul![
                        C![],
                        li![
                            C![],
                            div![
                                C![],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'm ",
                                span![
                                    C![],
                                    "more productive"
                                ],
                                " when I'm working ",
                                span![
                                    C![],
                                    "remotely"
                                ],
                                "."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I like to ",
                                span![
                                    C![],
                                    "help"
                                ],
                                " people (not only on GitHub) and to mentor juniors."
                            ]
                        ],
                        li![
                            C![],
                            div![
                                C![],
                                "▶\u{fe0e}"
                            ],
                            div![
                                "I'd rather think about your project for free in a gym than sit and wait for ideas.",
                                br![],
                                "I also recommend to read ",
                                a![
                                    attrs!{
                                        At::Href => "https://medium.com/@jsonpify/you-dont-need-standup-9a74782517c1"
                                    },
                                    C![],
                                    "You don’t need standup"
                                ],
                                "."
                            ]
                        ]
                    ]
                ]
            ]
        ],
        // Did you know section
        section![
            C![],
            h2![
                C![],
                "Did you know..."
            ],
            ul![
                C![],
                li![
                    C![],
                    div![
                        C![],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I programmed a real football cannon."
                    ]
                ],
                li![
                    C![],
                    div![
                        C![],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "I jumped off a plane",
                        br![],
                        "and a bridge."
                    ]
                ],
            ]
        ],
        // Want to meet section
        section![
            C![],
            div![
                C![],
                img![
                    C![],
                    attrs!{
                        At::Src => image_src("photo_2.jpg"),
                    }
                ],
            ],
            ul![
                C![],
                li![
                    C![],
                    div![
                        C![],
                        "▶\u{fe0e}"
                    ],
                    div![
                        "Want to meet somewhere in ",
                        span![
                            C![],
                            "Prague"
                        ],
                        "?",
                        br![],
                        "Is there good coffee, tea, sushi or some spicy food? Ok! ",
                        a![
                            attrs!{
                                At::Href => MAIL_TO_KAVIK,
                            },
                            C![],
                            "martin@kavik.cz"
                        ]
                    ]
                ],
            ]
        ],
        // Resume section
        section![
            C![],
            // Download my resume
            a![
                attrs!{
                    At::Href => asset_path("Martin_Kavik_resume.pdf")
                },
                C![],
                "Download my\u{00A0}",
                span![
                    C![],
                    "Resume"
                ],
                span![
                    C![],
                    ".pdf"
                ],
                img![
                    C![],
                    attrs!{
                        At::Src => image_src("download.svg")
                    }
                ],
            ],
            // My GitHub profile
            a![
                attrs!{
                    At::Href => "https://github.com/MartinKavik"
                },
                C![],
                "Go to my\u{00A0}",
                span![
                    C![],
                    "GitHub"
                ],
                "\u{00A0}profile",
                img![
                    C![],
                    attrs!{
                        At::Src => image_src("link_arrow.svg")
                    }
                ],
            ]
        ],
    ]
}
