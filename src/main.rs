mod app;
mod ui;
mod configuration;

use ui::display::render::init;

fn main() {
    init().unwrap();
}
