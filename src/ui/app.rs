use std::collections::HashSet;

use dioxus::prelude::*;

use crate::data::get_classes;
use crate::ui::{card_viewer, settings};

pub fn app(cx: Scope) -> Element {
    let view = use_state(cx, || View::CardViewer);
    let filtered_classes = use_state(cx, || {
        get_classes().iter().map(|c| c.id).collect::<HashSet<_>>()
    });

    let on_settings_click = move |_| {
        if view == &View::Settings {
            view.set(View::CardViewer);
        } else {
            view.set(View::Settings);
        }
    };

    let on_class_filter_toggle = move |e: crate::data::ClassId| {
        let mut new_filtered_classes = filtered_classes.get().clone();

        if filtered_classes.contains(&e) {
            new_filtered_classes.remove(&e);
        } else {
            new_filtered_classes.insert(e);
        };

        filtered_classes.set(new_filtered_classes);
    };

    let on_class_filter_select_all = move |_| {
        filtered_classes.set(get_classes().iter().map(|c| c.id).collect::<HashSet<_>>());
    };

    let on_class_filter_deselect_all = move |_| {
        filtered_classes.set(HashSet::new());
    };

    cx.render(rsx! {
        main {
            if view == &View::Settings {
                rsx!(settings::settings {
                    selected_classes: filtered_classes,
                    on_class_filter_toggle: on_class_filter_toggle,
                    on_class_filter_select_all: on_class_filter_select_all,
                    on_class_filter_deselect_all: on_class_filter_deselect_all,
                })
            } else if view == &View::CardViewer {
                rsx!(card_viewer {
                    selected_classes: filtered_classes
                })
            } else {
                rsx!("Error")
            }
            button {
                onclick: on_settings_click,
                "settings",
            }
        }
    })
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum View {
    CardViewer,
    Settings,
}
