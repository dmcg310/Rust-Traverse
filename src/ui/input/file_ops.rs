use super::{extract::*, run_app::Command};
use crate::{app::app::App, ui::display::block::block_binds};

pub fn handle_new_file(app: &mut App, input_active: &mut bool) {
    if app.files.state.selected().is_some() {
        if (*input_active == false && app.last_command != Some(Command::CreateFile))
            || (*input_active == true && app.last_command.is_none())
        {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::CreateFile);
        }
    } else if app.dirs.state.selected().is_some() {
        if (*input_active == false && app.last_command != Some(Command::CreateDir))
            || (*input_active == true && app.last_command.is_none())
        {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::CreateDir);
        }
    }
}

pub fn handle_delete(app: &mut App) {
    if let Some(selected) = app.files.state.selected() {
        if selected == 0 && app.files.items.len() == 0 {
            return;
        } else {
            let file = app.files.items[selected].0.clone();

            trash::delete(&file).unwrap();
            app.update_files();

            if selected >= app.files.items.len() {
                app.files
                    .state
                    .select(Some(app.files.items.len().saturating_sub(1)));
            }
        }
    } else if let Some(selected) = app.dirs.state.selected() {
        let dir = app.dirs.items[selected].0.clone();

        if dir == "../" {
            return;
        } else {
            trash::delete(&dir).unwrap();
            app.update_dirs();

            if selected >= app.dirs.items.len() {
                app.dirs
                    .state
                    .select(Some(app.dirs.items.len().saturating_sub(1)));
            }
        }
    }
}

pub fn handle_rename(app: &mut App, input: &mut String, input_active: &mut bool) {
    if block_binds(app) {
        return;
    }

    if app.files.state.selected().is_some() {
        if *input_active == false && app.last_command != Some(Command::RenameFile) {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::RenameFile);

            *input = app.files.items[app.files.state.selected().unwrap()]
                .0
                .clone();
        }
    } else if app.dirs.state.selected().is_some() {
        if app.dirs.items[app.dirs.state.selected().unwrap()].0 == "../" {
            return;
        } else {
            if *input_active == false && app.last_command != Some(Command::RenameDir) {
                *input_active = true;
                app.show_popup = true;
                app.last_command = Some(Command::RenameDir);
                *input = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();
            }
        }
    }
}

pub fn extract(app: &mut App) {
    if app.files.state.selected().is_some() {
        let file = app.files.items[app.files.state.selected().unwrap()]
            .0
            .clone();

        if file.ends_with(".tar.gz") {
            extract_tar(app, &file).expect("Failed to extract tar file");
        } else if file.ends_with(".zip") {
            extract_zip(app, &file).expect("Failed to extract zip file");
        }
    }
}

fn add_dir(app: &mut App) {
    let selected = app.dirs.state.selected().unwrap();
    let cur_dir = std::env::current_dir().unwrap();

    app.selected_files.push(format!(
        "{}/{}",
        cur_dir.display(),
        app.dirs.items[selected].0
    ));
}

fn add_file(app: &mut App) {
    let selected = app.files.state.selected().unwrap();
    let cur_dir = std::env::current_dir().unwrap();
    let selected = format!("{}/{}", cur_dir.display(), app.files.items[selected].0);

    for file in app.selected_files.clone() {
        if file == selected {
            return;
        }
    }

    app.selected_files.push(selected);
}

pub fn add_to_selected(app: &mut App) {
    if app.dirs.state.selected().is_some() {
        add_dir(app);
    } else if app.files.state.selected().is_some() {
        add_file(app);
    }
}

pub fn handle_paste_or_move(app: &mut App) {
    // TODO:
    // copying files into directories where they already exist
    // (error box maybe for global error handling)
    if app.selected_files.len() == 0 {
        if app.selected_dirs.len() == 0 {
            return;
        }
    }

    if let Some(selected) = app.ops_menu.state.selected() {
        let mut cur_dir = std::env::current_dir().unwrap();
        match selected {
            0 => {
                // copy
                for file in app.selected_files.clone() {
                    for cur_files in app.files.items.clone() {
                        if file == cur_files.0 {
                            continue;
                        }

                        std::process::Command::new("cp")
                            .arg("-r")
                            .arg(&file)
                            .arg(&cur_dir)
                            .spawn()
                            .expect("Failed to copy file");

                        app.show_ops_menu = false;
                        app.last_command = None;
                        app.selected_files = vec![];
                        app.selected_dirs = vec![];

                        app.update_files();
                        app.update_dirs();

                        app.files
                            .state
                            .select(Some(app.files.items.len().saturating_sub(1)));
                    }

                    cur_dir = std::env::current_dir().unwrap();
                }
            }
            1 => {
                // move
                for file in app.selected_files.clone() {
                    for cur_files in app.files.items.clone() {
                        if file == cur_files.0 {
                            continue;
                        }

                        std::process::Command::new("mv")
                            .arg(&file)
                            .arg(&cur_dir)
                            .spawn()
                            .expect("Failed to move file");

                        app.show_ops_menu = false;
                        app.last_command = None;
                        app.selected_files = vec![];
                        app.selected_dirs = vec![];

                        app.update_files();
                        app.update_dirs();

                        app.files
                            .state
                            .select(Some(app.files.items.len().saturating_sub(1)));

                        cur_dir = std::env::current_dir().unwrap();
                    }
                }
            }
            2 => {
                // clear selection
                app.last_command = None;
                app.show_ops_menu = false;

                app.selected_files = vec![];
                app.selected_dirs = vec![];

                app.update_files();
                app.update_dirs();
            }
            _ => {}
        }
    }
}
