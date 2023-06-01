use crate::ui::stateful_list::StatefulList;
use glob::glob;
use std::fs::read_dir;
use std::{fs, process::Command};

pub struct App {
    pub files: StatefulList<(String, String)>,
    pub dirs: StatefulList<(String, String)>,
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

        let mut dirs = StatefulList::with_items(vec![]);
        for entry in read_dir("./").unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_dir() {
                let temp = entry.file_name().into_string().unwrap();
                dirs.items.push((temp.clone(), temp));
            }
        }

        App { files, dirs }
    }
}
