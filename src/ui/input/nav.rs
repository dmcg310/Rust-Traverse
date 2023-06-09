use super::stateful_list::StatefulList;
use super::*;
use crate::app::app::App;
use run_app::Command;
use std::path::PathBuf;
use sublime_fuzzy::best_match;
use walkdir::WalkDir;

pub fn handle_nav(app: &mut App, input_active: &mut bool) {
    if *input_active == false {
        app.show_nav = true;
        *input_active = true;
        app.last_command = Some(Command::ShowNav);
    }
}

fn fzf(app: &mut App, input: &mut String) -> Vec<PathBuf> {
    let query = input.clone();
    let dir = app.cur_dir.clone();
    let dir = dir.trim_end_matches('\n');

    let mut result = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            if entry.path().to_str().unwrap().contains(".git") {
                // config
                continue;
            }

            let filename = entry.file_name().to_str().unwrap().to_string();

            if let Some(matched) = best_match(&query, &filename) {
                if matched.score() > 0 {
                    result.push(entry.path().to_path_buf());
                }
            }
        }
    }

    result
}

pub fn handle_fzf(app: &mut App, input: &mut String, input_active: &mut bool) {
    app.show_fzf = true;
    app.show_popup = true;
    app.last_command = Some(Command::ShowFzf);

    *input_active = true;

    let result = fzf(app, input);

    app.fzf_results = StatefulList::with_items(
        result
            .iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect(),
    );
}
