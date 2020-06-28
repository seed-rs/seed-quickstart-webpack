use seed::{prelude::*, *};
const ENTER_KEY: u32 = 13;

#[derive(Clone)]
pub enum Msg {
  Update(String),
  Clear,
}

pub struct Model {
  pub value: String,
  _sub_handle: SubHandle,
}

#[derive(Copy, Clone)]
pub struct Reset;

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
  Model {
    value: "".to_owned(),
    _sub_handle: orders.subscribe_with_handle(|_: Reset| Msg::Clear),
  }
}

pub fn update<Ms: 'static, GMs>(msg: Msg, model: &mut Model, _: &mut impl Orders<Ms, GMs>) {
  match msg {
    Msg::Update(val) => {
      model.value = val;
    }
    Msg::Clear => model.value = "".to_owned(),
  }
}

pub fn view<Ms: 'static>(
  model: &Model,
  on_enter_key: impl FnOnce() -> Ms + Clone + 'static,
  to_msg: impl FnOnce(Msg) -> Ms + Clone + 'static,
) -> Node<Ms> {
  input![
    class!["new-todo"],
    attrs! {At::Placeholder => "What needs to be done?", At::Value => model.value },
    input_ev(Ev::Input, move |val| to_msg(Msg::Update(val))),
    keyboard_ev(Ev::KeyDown, |keyboard_event| {
      IF!(keyboard_event.key_code() == ENTER_KEY => {
        on_enter_key()
      })
    })
  ]
}
