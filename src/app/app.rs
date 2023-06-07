use crate::ui::display::{pane::get_du, pane::get_pwd};
use crate::ui::input::{run_app::Command, stateful_list::StatefulList};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{ListState, Widget},
};
use std::fs::{self, read_dir, File};

pub struct App {
    pub files: StatefulList<(String, String)>,
    pub dirs: StatefulList<(String, String)>,
    pub content: StatefulList<String>,
    pub cur_du: String,
    pub cur_dir: String,
    pub show_popup: bool,
    pub show_nav: bool,
    pub show_fzf: bool,
    pub show_bookmark: bool,
    pub fzf_results: StatefulList<String>,
    pub selected_fzf_result: usize,
    pub selected_item_state: ListState,
    pub last_command: Option<Command>,
    pub bookmarked_dirs: StatefulList<String>,
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
            content: StatefulList::with_items(vec![]),
            show_popup: false,
            show_nav: false,
            show_fzf: false,
            show_bookmark: false,
            fzf_results: StatefulList::with_items(vec![]),
            selected_fzf_result: 0,
            selected_item_state: ListState::default(),
            last_command: None,
            bookmarked_dirs: StatefulList::with_items(vec![]),
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

    pub fn update_bookmarks(&mut self) {
        self.show_bookmark = true;
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
