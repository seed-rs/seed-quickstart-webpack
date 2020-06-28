use crate::component::*;
use crate::{Model, Msg};
use seed::{prelude::*, *};

pub fn view(text: String, model: &Model) -> Node<Msg> {
  header![
    class!["header"],
    h1![text],
    todo_input::view(&model.todo, || Msg::AddTodo, Msg::Input),
  ]
}
