use std::collections::HashSet;

use dioxus::prelude::*;

use super::{card_image, filter_input};
use crate::data::{search_by_name, ClassId};

const CARD_VIEWER_STYLES: &str = r"
    display: flex;
    flex-direction: column;
";

#[derive(Props)]
pub struct CardViewerProps<'a> {
    selected_classes: &'a HashSet<ClassId>,
}

pub fn card_viewer<'a>(cx: Scope<'a, CardViewerProps<'a>>) -> Element<'a> {
    let search_text = use_state(cx, String::new);

    let card_match = if search_text.is_empty() {
        None
    } else {
        search_by_name(search_text.get(), Some(cx.props.selected_classes))
    };

    let img_path = card_match
        .map(|c| "worldhaven/images/".to_owned() + c.image())
        .unwrap_or_default();

    cx.render(rsx! {
        div {
            style: "{CARD_VIEWER_STYLES}",
            card_image {
                image_path: "{img_path}"
            }
            filter_input {
                on_input: move |s| search_text.set(s)
            }
        }
    })
}
