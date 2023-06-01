use ratatui::{text::Spans, widgets::ListItem};
use std::path::Path;
use std::process::Command;

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
        let output = Command::new("ls")
            .arg("-ld")
            .arg(file)
            .output()
            .expect("failed to execute process");

        if output.stdout.is_empty() {
            return vec![ListItem::new(Spans::from("No directory selected"))];
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut items = Vec::new();

        for line in output_str.lines() {
            items.push(ListItem::new(Spans::from(line.to_string())));
        }

        return items;
    }

    if file.is_file() {
        let output = Command::new("ls")
            .arg("-lh")
            .arg(file)
            .output()
            .expect("failed to execute process");

        if output.stdout.is_empty() {
            return vec![ListItem::new(Spans::from("No file selected"))];
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut items = Vec::new();

        for line in output_str.lines() {
            items.push(ListItem::new(Spans::from(line.to_string())));
        }

        return items;
    }

    vec![ListItem::new(Spans::from("No file selected"))]
}
