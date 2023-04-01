mod data;
mod ui;

use dioxus_desktop::{Config, PhysicalSize, WindowBuilder};
use ui::app;

fn main() {
    let window_builder = WindowBuilder::new()
        .with_inner_size(PhysicalSize {
            width: 400,
            height: 642,
        })
        .with_title("GH Card Viewer");

    let cfg = Config::new().with_window(window_builder).with_custom_head(
        "<link rel='stylesheet' type='text/css' href='src/styles.css'></script>".to_string(),
    );

    dioxus_desktop::launch_with_props(app, (), cfg);
}
