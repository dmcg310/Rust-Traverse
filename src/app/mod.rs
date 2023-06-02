use crate::ui::{
    pane::{get_du, get_pwd},
    stateful_list::StatefulList, run_app::Command,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{ListState, Widget},
};
use std::fs::{self, read_dir, File};

pub struct App {
    pub files: StatefulList<(String, String)>,
    pub dirs: StatefulList<(String, String)>,
    pub cur_du: String,
    pub cur_dir: String,
    pub show_popup: bool,
    pub selected_item_state: ListState,
    pub last_command: Option<Command>,
}

impl App {
    pub fn new() -> App {
        let mut files = StatefulList::with_items(vec![]);
        for entry in read_dir("./").unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let temp = entry.file_name().into_string().unwrap();
                files.items.push((temp.clone(), temp));
            }
        }

        let mut dirs = StatefulList::with_items(vec![(("../".to_string(), "../".to_string()))]);
        for entry in read_dir("./").unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                let temp = entry.file_name().into_string().unwrap();
                dirs.items.push((temp.clone(), temp));
            }
        }

        let cur_dir = get_pwd();
        let cur_du = get_du();

        App {
            files,
            dirs,
            cur_du,
            cur_dir,
            show_popup: false,
            selected_item_state: ListState::default(),
            last_command: None,
        }
    }

    pub fn update_files(&mut self) {
        self.files.items.clear();
        for entry in read_dir("./").unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let temp = entry.file_name().into_string().unwrap();
                self.files.items.push((temp.clone(), temp));
            }
        }
    }

    pub fn update_dirs(&mut self) {
        self.dirs.items.clear();
        self.dirs.items.push(("../".to_string(), "../".to_string()));
        for entry in read_dir("./").unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                let temp = entry.file_name().into_string().unwrap();
                self.dirs.items.push((temp.clone(), temp));
            }
        }
    }

    pub fn create_file(input: &str) -> bool {
        if File::create(input).is_ok() {
            true
        } else {
            false
        }
    }

    pub fn create_dir(input: &str) -> bool {
        format!("./{}", input);
        if fs::create_dir(input).is_ok() {
            true
        } else {
            false
        }
    }
}

pub struct InputBox<'a> {
    text: &'a str,
    style: Style,
}

impl<'a> Widget for InputBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_stringn(area.x, area.y, self.text, area.width as usize, self.style);
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
