mod app;
mod ui;
use ui::render::render;

fn main() {
    render().unwrap();
}
