use crate::ui::{
    pane::{get_du, get_pwd},
    stateful_list::StatefulList,
};
use std::fs::read_dir;

pub struct App {
    pub files: StatefulList<(String, String)>,
    pub dirs: StatefulList<(String, String)>,
    pub cur_du: String,
    pub cur_dir: String,
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
}
