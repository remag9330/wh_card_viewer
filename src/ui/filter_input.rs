use dioxus::prelude::*;

const FILTER_INPUT_STYLES: &str = r"
    width: 100%;
    box-sizing: border-box;
";

#[derive(Props)]
pub struct FilterInputProps<'a> {
    on_input: EventHandler<'a, String>,
}

pub fn filter_input<'a>(cx: Scope<'a, FilterInputProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        input {
            style: "{FILTER_INPUT_STYLES}",
            "type": "text",
            oninput: move |e| cx.props.on_input.call(e.data.value.clone())
        }
    })
}
