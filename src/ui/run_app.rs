use super::render::render;
use crate::app::App;
use crate::ui::pane::get_pwd;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use distance::levenshtein;
use ratatui::backend::Backend;
use ratatui::terminal::Terminal;
use std::path::PathBuf;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(PartialEq)]
pub enum Command {
    CreateFile,
    CreateDir,
    RenameFile,
    RenameDir,
    ShowNav,
    ShowFzf,
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = std::time::Instant::now();
    let mut input = String::new();
    let mut input_active = false;

    loop {
        terminal.draw(|f| render(f, &mut app, &mut input))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('1') => {
                            handle_pane_switching(&mut app, &mut input, input_active, 1);
                        }
                        KeyCode::Char('2') => {
                            handle_pane_switching(&mut app, &mut input, input_active, 2);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            handle_movement(&mut app, &mut input, input_active, 'j');
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            handle_movement(&mut app, &mut input, input_active, 'k');
                        }
                        KeyCode::Char('n') => {
                            handle_new_file(&mut app, &mut input, &mut input_active, 'n');
                        }
                        KeyCode::Char('f') => {
                            handle_nav(&mut app, &mut input, &mut input_active, 'f');
                        }
                        KeyCode::Char('w') => {
                            handle_fzf(&mut app, &mut input, &mut input_active);
                        }
                        KeyCode::Char('d')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            handle_delete(&mut app);
                        }
                        KeyCode::Char('r') => {
                            handle_rename(&mut app, &mut input, &mut input_active);
                        }
                        KeyCode::Esc => {
                            if input_active {
                                input_active = false;
                                app.show_popup = false;
                                app.show_nav = false;
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Char(c) => {
                            if input_active {
                                input.push(c);

                                if app.last_command == Some(Command::ShowFzf) {
                                    handle_fzf(&mut app, &mut input, &mut input_active);
                                }
                            }
                        }
                        KeyCode::Enter => {
                            handle_submit(&mut app, &mut input, &mut input_active);
                        }
                        KeyCode::Backspace => {
                            if input_active {
                                input.pop();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

fn handle_pane_switching(app: &mut App, input: &mut String, input_active: bool, key: u8) {
    if input_active {
        input.push_str(&key.to_string());
    } else {
        if key == 1 {
            app.files.state.select(Some(0));
            app.dirs.state.select(None);
            app.content.state.select(None);
        } else if key == 2 {
            app.dirs.state.select(Some(0));
            app.files.state.select(None);
            app.content.state.select(None);
        }
    }
}

fn handle_movement(app: &mut App, input: &mut String, input_active: bool, key: char) {
    if input_active {
        input.push(key);
    }

    if app.files.state.selected().is_some() {
        if key == 'j' {
            app.files.next();
        } else {
            app.files.previous();
        }
    } else if app.dirs.state.selected().is_some() {
        if key == 'j' {
            app.dirs.next();
        } else {
            app.dirs.previous();
        }
    } else if app.content.state.selected().is_some() {
        if key == 'j' {
            app.content.next();
        } else {
            app.content.previous();
        }
    }
}

fn handle_new_file(app: &mut App, input: &mut String, input_active: &mut bool, key: char) {
    if app.files.state.selected().is_some() {
        if (*input_active == false && app.last_command != Some(Command::CreateFile))
            || (*input_active == true && app.last_command.is_none())
        {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::CreateFile);
        } else {
            input.push(key);
        }
    } else if app.dirs.state.selected().is_some() {
        if (*input_active == false && app.last_command != Some(Command::CreateDir))
            || (*input_active == true && app.last_command.is_none())
        {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::CreateDir);
        } else {
            input.push(key);
        }
    } else {
        input.push(key);
    }
}

fn handle_nav(app: &mut App, input: &mut String, input_active: &mut bool, key: char) {
    if *input_active == false {
        app.show_nav = true;
        *input_active = true;
        app.last_command = Some(Command::ShowNav);
    } else {
        input.push(key);
    }
}

fn handle_delete(app: &mut App) {
    if app.files.state.selected().is_some() {
        let file = app.files.items[app.files.state.selected().unwrap()]
            .0
            .clone();

        std::fs::remove_file(file).unwrap();
        app.update_files();
    } else if app.dirs.state.selected().is_some() {
        let dir = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();

        std::fs::remove_dir_all(dir).unwrap();
        app.update_dirs();
    }
}

fn handle_rename(app: &mut App, input: &mut String, input_active: &mut bool) {
    if app.files.state.selected().is_some() {
        if *input_active == false && app.last_command != Some(Command::RenameFile) {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::RenameFile);
            *input = app.files.items[app.files.state.selected().unwrap()]
                .0
                .clone();
        } else {
            input.push('r');
        }
    } else if app.dirs.state.selected().is_some() {
        if *input_active == false && app.last_command != Some(Command::RenameDir) {
            *input_active = true;
            app.show_popup = true;
            app.last_command = Some(Command::RenameDir);
            *input = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();
        } else {
            input.push('r');
        }
    } else {
        if *input_active {
            input.push('r');
        }
    }
}

fn handle_submit(app: &mut App, input: &mut String, input_active: &mut bool) {
    if *input_active {
        if app.last_command == Some(Command::CreateFile) {
            App::create_file(&input);
            app.update_files();
            app.last_command = None;
        } else if app.last_command == Some(Command::CreateDir) {
            App::create_dir(&input);
            app.update_dirs();
            app.last_command = None;
        } else if app.last_command == Some(Command::RenameFile) {
            let file = app.files.items[app.files.state.selected().unwrap()]
                .0
                .clone();

            std::fs::rename(file, input.clone()).unwrap();
            app.update_files();
            app.last_command = None;
        } else if app.last_command == Some(Command::RenameDir) {
            let dir = app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();

            std::fs::rename(dir, input.clone()).unwrap();
            app.update_dirs();
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
            } else if app.last_command == Some(Command::ShowFzf) {
                // app.show_popup = false;
                // app.update_dirs();
                // app.update_files();
                // app.last_command = None;
            } else {
                app.show_popup = false;
                app.show_nav = false;
                app.last_command = None;
            }
        }

        input.clear();
        app.show_popup = false;
        *input_active = false;
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

fn fzf(app: &mut App, input: &mut String, input_active: &mut bool) -> Vec<PathBuf> {
    let query = input.clone();
    let dir = app.cur_dir.clone();
    let dir = dir.trim_end_matches('\n');

    let mut result = Vec::new();

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            let filename = entry.file_name().to_str().unwrap().to_string();
            let dist = levenshtein(&query, &filename);

            if dist < 5 {
                result.push(entry.path().to_path_buf());
            }
        }
    }

    result
}

fn handle_fzf(app: &mut App, input: &mut String, input_active: &mut bool) {
    app.show_fzf = true;
    app.show_popup = true;
    app.last_command = Some(Command::ShowFzf);

    *input_active = true;

    let result = fzf(app, input, input_active);

    app.fzf_results = result
        .into_iter()
        .map(|x| x.display().to_string())
        .collect();
}
