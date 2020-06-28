use seed::{prelude::*, *};

use crate::component::Button;
use crate::*;
use strum::IntoEnumIterator;

pub fn view(model: &Model) -> Vec<Node<Msg>> {
  let filter: Vec<Node<Msg>> = Filter::iter()
    .map(|flt| {
      li![a![
        class!["not-selected"],
        attrs! {At::Href => &flt.to_string()},
        flt.to_string()
      ]]
    })
    .collect();
  vec![
    span![
      class!["todo-count"],
      strong![model.todos.len().to_string()],
      " item(s) left"
    ],
    ul![class!["filters"], filter],
    Button(
      Some(format!("Clear completed ({})", 5)),
      Some("clear-completed"),
    ),
  ]
}
