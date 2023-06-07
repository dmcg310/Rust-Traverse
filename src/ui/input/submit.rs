use super::*;
use crate::app::app::App;
use crate::ui::display::pane::get_pwd;
use run_app::Command;
use std::path::PathBuf;

pub fn handle_submit(app: &mut App, input: &mut String, input_active: &mut bool) {
    if *input_active {
        if app.last_command == Some(Command::CreateFile) {
            App::create_file(&input);
            app.update_files();
            app.update_dirs();
            app.last_command = None;
        } else if app.last_command == Some(Command::CreateDir) {
            App::create_dir(&input);
            app.update_dirs();
            app.update_files();
            app.last_command = None;
        } else if app.last_command == Some(Command::RenameFile) {
            let file = app.files.items[app.files.state.selected().unwrap()]
                .0
                .clone();

            std::fs::rename(file, input.clone()).unwrap();
            app.update_files();
            app.update_dirs();
            app.last_command = None;
        } else if app.last_command == Some(Command::RenameDir) {
            let dir = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();

            std::fs::rename(dir, input.clone()).unwrap();
            app.update_dirs();
            app.update_files();
            app.last_command = None;
        } else if app.last_command == Some(Command::ShowNav) {
            let path = Some(PathBuf::from(input.clone()));

            if path.is_some() {
                std::env::set_current_dir(path.unwrap()).unwrap();

                app.cur_dir = std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                app.update_files();
                app.update_dirs();

                app.show_popup = false;
                app.show_nav = false;
                app.last_command = None;
            } else {
                app.show_popup = false;
                app.show_nav = false;
                app.last_command = None;
            }
        }

        input.clear();
        app.show_popup = false;
        *input_active = false;
        app.update_files();
        app.update_dirs();
    } else {
        if app.dirs.state.selected().is_some() {
            if app.dirs.items[app.dirs.state.selected().unwrap()].0 == "../" {
                let mut path = std::env::current_dir().unwrap();
                path.pop();

                std::env::set_current_dir(path).unwrap();
                app.cur_dir = get_pwd();
            } else {
                let dir = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();

                std::env::set_current_dir(dir).unwrap();
                app.cur_dir = get_pwd();
            }
            app.update_files();
            app.update_dirs();

            if let Some(selected) = app.files.state.selected() {
                if selected >= app.files.items.len() {
                    if !app.files.items.is_empty() {
                        app.files
                            .state
                            .select(Some(app.files.items.len().saturating_sub(1)));
                    } else {
                        app.files.state.select(None);
                    }
                }
            }
            app.dirs.state.select(Some(0));
        }
    }
}

pub fn handle_open_fzf_result(app: &mut App, input: &mut String, input_active: &mut bool) {
    if app.fzf_results.state.selected().is_none() {
        return;
    } else {
        if app.fzf_results.items[app.fzf_results.state.selected().unwrap()]
            .clone()
            .is_ascii()
        {
            let path = app.fzf_results.items[app.fzf_results.state.selected().unwrap()].clone();
            let path = PathBuf::from(path).parent().unwrap().to_path_buf();
            std::env::set_current_dir(path).unwrap();

            app.update_files();
            app.update_dirs();

            app.show_fzf = false;
            app.show_popup = false;
            app.last_command = None;

            input.clear();
            *input_active = false;

            app.fzf_results.state.select(None);
            app.selected_fzf_result = 0;

            app.files.state.select(Some(0));
            app.dirs.state.select(None);

            app.cur_dir = get_pwd();
        }
    }
}

pub fn handle_open_bookmark(app: &mut App) {
    if app.bookmarked_dirs.state.selected().is_none() {
        return;
    } else {
        if app.bookmarked_dirs.items[app.bookmarked_dirs.state.selected().unwrap()]
            .clone()
            .is_ascii()
        {
            let path =
                app.bookmarked_dirs.items[app.bookmarked_dirs.state.selected().unwrap()].clone();
            let path = PathBuf::from(path);
            std::env::set_current_dir(path).unwrap();

            app.update_files();
            app.update_dirs();

            app.show_bookmark = false;
            app.show_popup = false;
            app.last_command = None;

            app.files.state.select(Some(0));
            app.dirs.state.select(None);

            app.cur_dir = get_pwd();
        }
    }
}
