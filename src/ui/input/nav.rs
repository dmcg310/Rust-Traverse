use super::stateful_list::StatefulList;
use super::*;
use crate::app::app::App;
use crate::ui::display::pane::get_pwd;
use crossterm::{
    cursor::MoveTo, cursor::Show, execute, style::Print, style::ResetColor, terminal::Clear,
    terminal::ClearType,
};
use run_app::Command;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use sublime_fuzzy::best_match;
use walkdir::WalkDir;

pub fn handle_nav(app: &mut App, input_active: &mut bool) {
    if !*input_active {
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
            let mut should_exclude = false;

            for dir in &app.excluded_directories {
                if entry.path().to_str().unwrap().contains(dir) {
                    should_exclude = true;
                    break;
                }
            }

            if should_exclude {
                continue;
            }

            if entry.path().to_str().unwrap().contains(".git") || !app.show_hidden {
                if !app.show_hidden {
                    if entry.file_name().to_str().unwrap().starts_with('.') {
                        continue;
                    }
                } else {
                    continue;
                }
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

pub fn abbreviate_path(path: &str) -> String {
    let components: Vec<&str> = path.split("/").collect();
    if components.len() > 4 {
        let last_three: Vec<&str> = components.into_iter().rev().take(3).collect();
        format!(
            ".../{}",
            last_three
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("/")
        )
    } else {
        path.to_string()
    }
}

pub fn output_cur_dir() {
    crossterm::terminal::disable_raw_mode().unwrap();

    let dir = get_pwd();

    execute!(
        stdout(),
        Clear(ClearType::All),
        ResetColor,
        Show,
        MoveTo(0, 0),
        Print(format!(
            "To navigate to traverse's last directory: cd {}",
            dir
        ))
    )
    .unwrap();

    stdout().flush().unwrap();
    exit(0);
}
