use seed::{prelude::*, *};

use crate::Msg;

pub fn view(
  header: Node<Msg>,
  body: Vec<Node<Msg>>,
  footer: Vec<Node<Msg>>,
  info: Vec<Node<Msg>>,
) -> Node<Msg> {
  div![
    class!["todomvc-wrapper"],
    section![
      class!["todoapp"],
      header,
      section![class!["main"], body],
      footer![class!["footer"], footer]
    ],
    footer![class!["info"], info]
  ]
}
