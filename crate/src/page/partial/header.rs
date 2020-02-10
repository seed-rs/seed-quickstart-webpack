use crate::{
    asset_path,
    generated::css_classes::C,
    image_src, Model, Msg, Page, ScrollHistory,
    Visibility::{self, *},
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view(model: &Model) -> impl View<Msg> {
    div!["header"]
}
