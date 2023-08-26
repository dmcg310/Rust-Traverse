use crate::app::app::App;
use dirs::config_dir;
use std::fs;
use std::io::BufRead;
use std::io::Write;

pub fn read_config(app: &mut App) {
    let config_path = config_dir().unwrap().join("traverse/config.txt");

    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap_or_else(|_| {
                    panic!("Failed to create directory at {}", parent.display())
                });
            }
        }

        let file = fs::File::create(&config_path).unwrap_or_else(|_| {
            panic!("Failed to create config file at {}", config_path.display())
        });

        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(b"show_hidden=false").unwrap();
        writer
            .write_all(b"\nexcluded_directories=.git,.idea,.vscode,target")
            .unwrap();
    }

    let file = fs::File::open(config_path).unwrap();
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

        if line.contains("excluded_directories") {
            let mut split = line.split("=");
            let value = split.nth(1).unwrap().trim().to_string();

            if value.contains(',') {
                let values = value.split(",");

                for val in values {
                    app.excluded_directories.push(val.trim().to_string());
                }
            } else {
                app.excluded_directories.push(value);
            }
        }
    }
}
