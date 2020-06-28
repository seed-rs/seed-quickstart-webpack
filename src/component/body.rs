use crate::*;
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Vec<Node<Msg>> {
  let my_data = &model.todos;

  let todo_list: Vec<Node<Msg>> = my_data
  .iter()
  //  .filter(|e| self.model.filter.fit(e))
  //  .enumerate()
   .map(|todo| li![class!["todo"],div![class!["view"],
   input![class!["toggle"],
   attrs!{At::Type => "checkbox", At::Checked => false.as_at_value()}],
    label![&todo.description],
    button![class!["destroy"]]
   ]]).collect();
  vec![
    input![
      attrs! {At::Type => "checkbox"; At::Checked => true.as_at_value()},
      class!["toggle-all"]
    ],
    ul![class!["todo-list"], todo_list],
  ]
}
