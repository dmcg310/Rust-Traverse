mod app;
mod ui;
use ui::render::render;

fn main() {
    Some(render()).expect("Failed to render");
}
