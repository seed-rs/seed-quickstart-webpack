use seed::{*, prelude::*};

use crate::Msg;

pub fn view(label:Option<String>, class: Option<&str>/*,trigger: impl  Into<Ev>, message:Msg*/
) -> Node<Msg> {
    button![
      C![class.unwrap_or_default()],
      // simple_ev(trigger, message),
      label.unwrap_or("Button".to_owned())
    ]
}