mod component;
mod model;
mod view;
mod css_classes;

use crate::component::*;
use crate::view::*;
use crate::model::*;

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

fn view(model: &Model) -> Node<Msg> {
  page::view(
    header::view("todos".to_owned(), model),
    Body(&model),
    Footer(&model),
    vec![
      p!["Double-click to edit a todo".to_owned()],
      p![
        "Written by ".to_owned(),
        a![
          attrs! {At::Href => "https://github.com/fattenap/", At::Target => "_blank" },
          "Frank Panetta"
        ]
      ],
      p![
        "Part of ".to_owned(),
        a![
          attrs! {At::Href => "http://todomvc.com/", At::Target => "_blank" },
          "TodoMVC"
        ]
      ],
    ],
  )
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
  App::start("app", init, update, view);
}
