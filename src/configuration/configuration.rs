use crate::app::app::App;
use dirs::config_dir;
use std::io::BufRead;

pub fn read_config(app: &mut App) {
    if !config_dir().unwrap().join("traverse/config.txt").exists() {
        return;
    }

    let file = std::fs::File::open(config_dir().unwrap().join("traverse/config.txt")).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("show_hidden") {
            let mut split = line.split("=");
            let value = split.nth(1).unwrap().trim().to_string();

            if value.eq_ignore_ascii_case("true") {
                app.show_hidden = true;
            } else {
                app.show_hidden = false;
            }
        }
    }
}
