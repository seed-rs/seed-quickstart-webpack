#![allow(
    clippy::used_underscore_binding,
    clippy::non_ascii_literal,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::wildcard_imports,
    clippy::match_wildcard_for_single_variants
)]

mod css_classes;
mod page;

use crate::css_classes::C;
use seed::{prelude::*, *};

const STATIC_PATH: &str = "assets";
const IMAGES_PATH: &str = "assets/images";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";

const EXAMPLE: &str = "example";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_base_url(),
        counter: page::counter::init(&mut orders.proxy(Msg::Counter)),
        page: Page::init(url),
        menu_visible: false,
        in_prerendering: is_in_prerendering(),
    }
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    counter: page::counter::Model,
    page: Page,
    menu_visible: bool,
    pub in_prerendering: bool,
}

// ------ Page ------
#[derive(PartialEq)]
enum Page {
    Home,
    Example,
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(EXAMPLE) => Self::Example,
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn counter(self) -> page::counter::Urls<'a> {
        page::counter::Urls::new(self.base_url().add_path_part(EXAMPLE))
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    Counter(page::counter::Msg),
    ResetCounter,
    ToggleMenu,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
        Msg::Counter(msg) => page::counter::update(msg, &mut model.counter),
        Msg::ResetCounter => {
            orders.notify(page::counter::DoReset);
        }
        Msg::ToggleMenu => model.menu_visible = !model.menu_visible,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        navbar(model),
        match &model.page {
            Page::Home => section![
                C![
                    C.fade_in,
                    C.hero,
                    C.is_primary,
                    C.is_bold,
                    C.is_fullheight_with_navbar
                ],
                figure![
                    C![C.image],
                    img![
                        C![C.gear_bgd, C.blur, C.rotate],
                        attrs! {
                          At::Width => "70%", At::Height => "70%"
                          At::Src => image_src("gear.svg")
                        }
                    ],
                ],
                div![
                    C![C.hero_body],
                    div![
                        C![C.container, C.has_text_centered],
                        h1![
                            format!("Counter is {}", model.counter.value),
                            C![C.title C.is_1]
                        ],
                        h2![
                            "Navigate to the Example page and change",
                            C![C.subtitle]
                        ],
                        button![
                            C![
                                C.button,
                                C.is_primary,
                                C.is_inverted,
                                C.is_outlined
                            ],
                            "Reset Counter",
                            ev(Ev::Click, |_| Msg::ResetCounter),
                            ev(Ev::Click, |_| log!("Reset counter!")),
                        ]
                    ]
                ],
            ],
            Page::Example => {
                page::counter::view(&model.counter).map_msg(Msg::Counter)
            }
            Page::NotFound => page::not_found::view(),
        },
    ]
}

fn navbar(model: &Model) -> Node<Msg> {
    let base_url: &Url = &model.base_url;
    let current_page: &Page = &model.page;

    nav![
        C![C.navbar, C.is_light],
        div![
            C![C.navbar_brand],
            a![
                C![C.navbar_item],
                attrs! { At::Href => "https://seed-rs.org/"},
                img![
                    attrs! {
                      At::Src => image_src("seed_logo.svg"), At::Alt => "Logo"
                    },
                    style! {St::Height => rem(2)}
                ]
            ],
            span![
                C![C.navbar_burger, IF!(model.menu_visible => C.is_active)],
                ev(Ev::Click, |_| Msg::ToggleMenu),
                span![],
                span![],
                span![],
            ]
        ],
        div![
            C![C.navbar_menu, IF!(model.menu_visible => C.is_active)],
            div![
                C![C.navbar_start],
                a![
                    C![
                        C.navbar_item,
                        IF!(*current_page == Page::Home => C.is_active)
                    ],
                    attrs! { At::Href => Urls::new(base_url).home() },
                    "Home"
                ],
                a![
                    C![
                        C.navbar_item,
                        IF!(*current_page == Page::Example => C.is_active)
                    ],
                    attrs! { At::Href => Urls::new(base_url).counter().counter_url()},
                    "Example"
                ],
                a![
                    C![C.navbar_item],
                    "Documentation",
                    attrs! {At::Href => "https://docs.rs/crate/seed/0.7.0"}
                ],
            ],
            div![
                C![C.navbar_end],
                span![
                    C![C.navbar_item],
                    a![
                        attrs! {At::Href => "https://github.com/seed-rs/seed"},
                        span![
                            C![C.icon, C.is_medium, C.has_text_dark],
                            i![C![C.fab, C.fa_2x, C.fa_github]]
                        ],
                    ]
                ]
            ]
        ]
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
