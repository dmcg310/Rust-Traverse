use ratatui::{text::Spans, widgets::ListItem};
use std::path::Path;
use std::process::Command;
use sysinfo::{DiskExt, System, SystemExt};

#[allow(dead_code)]
enum PaneState {
    Selected,
    NotSelected,
}

#[allow(dead_code)]
struct SelectedPane<T> {
    pub state: PaneState,
    pub items: Vec<T>,
}

pub fn selected_pane_content(input: &String) -> Vec<ListItem<'static>> {
    let file = Path::new(&input);

    if let Some(ext) = file.extension() {
        if ext == "png" || ext == "jpg" {
            let output = Command::new("file")
                .arg(file)
                .output()
                .expect("failed to execute process");

            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut items = Vec::new();

            for line in output_str.lines() {
                items.push(ListItem::new(Spans::from(line.to_string())));
            }

            return items;
        }

        if ext == "mp4" || ext == "mp3" {
            let output = Command::new("ffprobe")
                .arg(file)
                .output()
                .expect("failed to execute process");

            if output.stdout.is_empty() {
                return vec![ListItem::new(Spans::from("Cannot get details of file"))];
            }

            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut items = Vec::new();

            for line in output_str.lines() {
                items.push(ListItem::new(Spans::from(line.to_string())));
            }

            return items;
        }
    }

    if file.is_dir() {
        let mut items = Vec::new();
        let output = Command::new("ls")
            .arg("-ld")
            .arg(file)
            .output()
            .expect("failed to execute process");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let output_vec = output_str.split_whitespace().collect::<Vec<&str>>();

        let perms = output_vec[0];
        let owner = output_vec[2];
        let size = output_vec[4];
        let date = output_vec[5];
        let day = output_vec[6];
        let time = output_vec[7];

        if output.stdout.is_empty() {
            return vec![ListItem::new(Spans::from("No directory selected"))];
        }

        #[allow(unused_variables)]
        for line in output_str.lines() {
            items.push(ListItem::new(Spans::from(format!(
                "{}  {}  {}  {} {}/{}",
                perms, owner, size, date, day, time
            ))));
        }

        return items;
    }

    if file.is_file() {
        let mut items = Vec::new();
        let output = Command::new("ls")
            .arg("-lh")
            .arg(file)
            .output()
            .expect("failed to execute process");

        let output_str = String::from_utf8_lossy(&output.stdout);
        let output_vec: Vec<&str> = output_str.split_whitespace().collect();

        if output.stdout.is_empty() {
            return vec![ListItem::new(Spans::from("No file selected"))];
        }

        let perms = output_vec[0];
        let owner = output_vec[2];
        let size = output_vec[4];
        let date = output_vec[5];
        let day = output_vec[6];
        let time = output_vec[7];

        #[allow(unused_variables)]
        for line in output_str.lines() {
            items.push(ListItem::new(Spans::from(format!(
                "{}  {}  {}  {} {}/{}",
                perms, owner, size, date, day, time
            ))));
        }
        return items;
    }

    vec![ListItem::new(Spans::from("No file selected"))]
}

pub fn get_pwd() -> String {
    let output = Command::new("pwd")
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8_lossy(&output.stdout);
    format!("{}", output_str)
}

pub fn get_du() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    if let Some(disk) = sys.disks().get(0) {
        let total = disk.total_space();
        let free = disk.available_space();
        let used = total - free;

        return format!(
            "{} used / {} total / {} free ",
            convert_bytes(used),
            convert_bytes(total),
            convert_bytes(free),
        );
    } else {
        return String::from("No disk found");
    }
}

fn convert_bytes(bytes: u64) -> String {
    let mut bytes = bytes;
    let mut unit = 0;

    while bytes > 1024 {
        bytes /= 1024;
        unit += 1;
    }

    let unit = match unit {
        0 => "B",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        _ => "PB",
    };

    format!("{} {}", bytes, unit)
}
