mod component;
mod css_classes;
mod model;
mod view;

use crate::component::*;
use crate::css_classes::C;
use crate::model::*;
use crate::view::*;

const STATIC_PATH: &str = "assets";
const IMAGES_PATH: &str = "assets/images";

use seed::{prelude::*, *};
use strum_macros::{EnumIter, ToString};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
  Model {
    todos: vec![],
    current_todo: None,
    editing: false,
    filter: Filter::All,
    todo: todo_input::init(&mut orders.proxy(Msg::Input)),
  }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Eq)]
pub enum Filter {
  All,
  Active,
  Completed,
}

pub struct Model {
  pub todos: Vec<TodoModel>,
  pub current_todo: Option<TodoModel>,
  pub editing: bool,
  pub filter: Filter,
  pub todo: todo_input::Model,
}

#[derive(Clone)]
pub enum Msg {
  Input(todo_input::Msg),
  AddTodo,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    Msg::Input(msg) => todo_input::update(msg, &mut model.todo, orders),
    Msg::AddTodo => {
      // maybe add a dirty/hasValue flag to the todo_input rather then check the len
      if &model.todo.value.len() > &0 {
        let entry = TodoModel {
          description: model.todo.value.to_owned(),
          completed: false,
        };
        model.todos.push(entry);
        orders.notify(todo_input::Reset);
      }
    }
  }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
  vec![
    section![
      C![C.hero, C.is_primary, C.is_bold, C.is_fullheight_with_navbar],
      div![
        C![C.hero_head],
        header![
          C![C.navbar],
          div![
            C![C.container],
            div![
              C![C.navbar_brand],
              a![
                C![C.navbar_item],
                img![
                  attrs! {
                    At::Src => image_src("seed_logo.svg"), At::Alt => "Logo"
                  },
                  style! {St::Height => rem(2)}
                ]
              ],
              span![
                C![C.navbar_burger],
                attrs! {At::Data => "navbarMenu"},
                span![],
                span![],
                span![],
              ]
            ],
            div![
              attrs! {At::Id => "navbarMenu"},
              C![C.navbar_menu],
              div![
                C![C.navbar_end],
                a![C![C.navbar_item, C.is_active], "Home"],
                a![C![C.navbar_item, C.is_active], "Examples"],
                a![C![C.navbar_item, C.is_active], "Documentation"],
                span![
                  C![C.navbar_item],
                  a![
                    C![C.button, C.is_success, C.is_inverted],
                    span![C![C.icon], i![C![C.fab, C.fa_github]]],
                    span!["Download"]
                  ]
                ]
              ]
            ]
          ]
        ]
      ],
      div![
        C![C.hero_body],
        div![
          C![C.container, C.has_text_centered],
          h1!["Coding highlights", C![C.title C.is_1]],
          h2!["Like your favorites", C![C.subtitle]]
        ]
      ],
      div![
        C![C.hero_foot],
        div![
          C![C.columns],
          div![
            C![C.column],
            div![
              C![C.card],
              div![
                C![C.card_content],
                h2![C![C.title], "Card Title"],
                h3![C![C.subtitle], "Card Title"],
              ],
              footer![
                C![C.card_footer],
                span![
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_success],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_up]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_danger],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_down]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_info],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_retweet]]
                  ],
                ]
              ]
            ]
          ],
          div![
            C![C.column],
            div![
              C![C.card],
              div![
                C![C.card_content],
                h2![C![C.title], "Card Title"],
                h3![C![C.subtitle], "Card Title"],
              ],
              footer![
                C![C.card_footer],
                span![
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_success],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_up]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_danger],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_down]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_info],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_retweet]]
                  ],
                ]
              ]
            ]
          ],
          div![
            C![C.column],
            div![
              C![C.card],
              div![
                C![C.card_content],
                h2![C![C.title], "Card Title"],
                h3![C![C.subtitle], "Card Title"],
              ],
              footer![
                C![C.card_footer],
                span![
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_success],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_up]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_danger],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_thumbs_down]]
                  ],
                  C![C.card_footer_item],
                  a![
                    C![C.button, C.is_info],
                    attrs! {At::Href => "#"},
                    i![C![C.fa, C.fa_retweet]]
                  ],
                ]
              ]
            ]
          ]
        ]
      ],
    ], // page::view(
       //   header::view("todos".to_owned(), model),
       //   Body(&model),
       //   Footer(&model),
       //   vec![
       //     p!["Double-click to edit a todo".to_owned()],
       //     p![
       //       "Written by ".to_owned(),
       //       a![
       //         attrs! {At::Href => "https://github.com/fattenap/", At::Target => "_blank" },
       //         "Frank Panetta"
       //       ]
       //     ],
       //     p![
       //       "Part of ".to_owned(),
       //       a![
       //         attrs! {At::Href => "http://todomvc.com/", At::Target => "_blank" },
       //         "TodoMVC"
       //       ]
       //     ],
       //   ],
       // )
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

#[wasm_bindgen(start)]
pub fn start() {
  App::start("app", init, update, view);
}

// @media screen and (min-width: 769px), print
// .columns:not(.is-desktop) {
  // display: flex;
// }

// C.is_desktop