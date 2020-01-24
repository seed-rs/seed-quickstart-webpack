// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::must_use_candidate)]

mod generated;
mod page;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, *};
use Visibility::*;

const TITLE_SUFFIX: &str = "Kavik.cz";
// https://mailtolink.me/
const MAIL_TO_KAVIK: &str = "mailto:martin@kavik.cz?subject=Something%20for%20Martin&body=Hi!%0A%0AI%20am%20Groot.%20I%20like%20trains.";
const MAIL_TO_HELLWEB: &str =
    "mailto:martin@hellweb.app?subject=Hellweb%20-%20pain&body=Hi!%0A%0AI%20hate";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//     Model
// ------ ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub struct Model {
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
}

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::About => "/about",
            Self::NotFound => "/404",
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        match url.path.first().map(String::as_str) {
            None | Some("") => Self::Home,
            Some("about") => Self::About,
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::UpdatePageTitle);

    let model = Model {
        page: url.into(),
        scroll_history: ScrollHistory::new(),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
    };

    AfterMount::new(model).url_handling(UrlHandling::None)
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//    Routes
// ------ ------

pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url))
}

// ------ ------
// Window Events
// ------ ------

pub fn window_events(_: &Model) -> Vec<Listener<Msg>> {
    vec![raw_ev(Ev::Scroll, |_| {
        // Some browsers use `document.body.scrollTop`
        // and other ones `document.documentElement.scrollTop`.
        let mut position = body().scroll_top();
        if position == 0 {
            position = document()
                .document_element()
                .expect("cannot get document element")
                .scroll_top()
        }
        Msg::Scrolled(position)
    })]
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Url),
    UpdatePageTitle,
    ScrollToTop,
    Scrolled(i32),
    ToggleMenu,
    HideMenu,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(url) => {
            model.page = url.into();
            orders.send_msg(Msg::UpdatePageTitle);
        },
        Msg::UpdatePageTitle => {
            let title = match model.page {
                Page::Home => TITLE_SUFFIX.to_owned(),
                Page::About => format!("About - {}", TITLE_SUFFIX),
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled(position) => {
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        },
    }
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

pub fn view(model: &Model) -> impl View<Msg> {
    // @TODO: Setup `prerendered` properly once https://github.com/David-OConnor/seed/issues/223 is resolved
    let prerendered = true;
    div![
        class![
            C.fade_in => !prerendered,
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view().els(),
            Page::About => page::about::view().els(),
            Page::NotFound => page::not_found::view().els(),
        },
        page::partial::header::view(model).els(),
        page::partial::footer::view().els(),
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(routes)
        .window_events(window_events)
        .build_and_start();

    log!("App started.");
}
