use std::collections::HashSet;

use dioxus::prelude::*;

use crate::data::{get_classes, Class, ClassId};

const SETTINGS_STYLE: &str = r"
    display: flex;
    flex-wrap: wrap;
";

#[derive(Props)]
pub struct SettingsProps<'a> {
    selected_classes: &'a HashSet<ClassId>,
    on_class_filter_toggle: EventHandler<'a, ClassId>,
    on_class_filter_select_all: EventHandler<'a>,
    on_class_filter_deselect_all: EventHandler<'a>,
}

pub fn settings<'a>(cx: Scope<'a, SettingsProps<'a>>) -> Element<'a> {
    let all_classes = get_classes();

    let on_class_filter_toggle = &cx.props.on_class_filter_toggle;
    let on_class_filter_select_all = &cx.props.on_class_filter_select_all;
    let on_class_filter_deselect_all = &cx.props.on_class_filter_deselect_all;

    cx.render(rsx! {
        div {
            style: "{SETTINGS_STYLE}",
            all_classes.into_iter().map(|c| {
                let selected = cx.props.selected_classes.contains(&c.id);
                rsx!(class_filter_selector {
                    class: c,
                    selected: selected,
                    on_class_filter_toggle: move |e| on_class_filter_toggle.call(e)
                })
            })
        }

        div {
            button {
                onclick: |_| on_class_filter_select_all.call(()),
                "Select All",
            }
            button {
                onclick: |_| on_class_filter_deselect_all.call(()),
                "Deselect All",
            }
        }
    })
}

const CLASS_FILTER_SELECTOR_STYLE: &str = r"
    width: 36px;
    height: 32px;
";

// const CLASS_FILTER_SELECTOR_IMG_STYLE: &str = r"
//     width: 100%;
//     height: 100%;
// ";

const CLASS_FILTER_SELECTOR_IMG_STYLE_SELECTED: &str = r"
    max-width: 100%;
    max-height: 100%;
    opacity: 1;
";

const CLASS_FILTER_SELECTOR_IMG_STYLE_UNSELECTED: &str = r"
    max-width: 100%;
    max-height: 100%;
    opacity: 0.7;
";

#[derive(Props)]
struct ClassFilterSelectorProps<'a> {
    class: Class,
    selected: bool,
    on_class_filter_toggle: EventHandler<'a, ClassId>,
}

fn class_filter_selector<'a>(cx: Scope<'a, ClassFilterSelectorProps<'a>>) -> Element<'a> {
    let class = &cx.props.class;
    let selected = cx.props.selected;
    let on_class_filter_toggle = &cx.props.on_class_filter_toggle;

    let style = if selected {
        CLASS_FILTER_SELECTOR_IMG_STYLE_SELECTED
    } else {
        CLASS_FILTER_SELECTOR_IMG_STYLE_UNSELECTED
    };

    cx.render(rsx! {
        button {
            style: "{CLASS_FILTER_SELECTOR_STYLE}",
            onclick: move |_| on_class_filter_toggle.call(class.id),
            img {
                src: "{class.icon_path}",
                style: style
            }
        }
    })
}
