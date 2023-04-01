use dioxus::prelude::*;

const CARD_IMAGE_STYLES: &str = r"
    width: 400px;
    height: 600px;
";

#[derive(Props)]
pub struct CardImageProps<'a> {
    image_path: &'a str,
}

pub fn card_image<'a>(cx: Scope<'a, CardImageProps<'a>>) -> Element<'a> {
    let img_path = cx.props.image_path;

    cx.render(rsx! {
        img {
            style: "{CARD_IMAGE_STYLES}",
            "src": "{img_path}",
        }
    })
}
