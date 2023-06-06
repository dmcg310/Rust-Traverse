use super::render::render;
use super::stateful_list::StatefulList;
use crate::app::App;
use crate::ui::pane::get_pwd;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use distance::levenshtein;
use ratatui::backend::Backend;
use ratatui::terminal::Terminal;
use std::path::PathBuf;
use std::time::Duration;
use trash;
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
                        KeyCode::Char('n')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            handle_fzf_movement(&mut app, 1);
                        }
                        KeyCode::Char('n') => {
                            handle_new_file(&mut app, &mut input, &mut input_active, 'n');
                        }
                        KeyCode::Char('p')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            handle_fzf_movement(&mut app, -1);
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
                            if app.show_popup || app.show_nav || app.show_fzf {
                                input_active = false;
                                app.show_popup = false;
                                app.show_nav = false;
                                app.show_fzf = false;
                                app.last_command = None;
                                input.clear();
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
                            if app.show_fzf {
                                handle_open_fzf_result(&mut app, &mut input, &mut input_active);
                            } else {
                                handle_submit(&mut app, &mut input, &mut input_active);
                            }
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

fn handle_fzf_movement(app: &mut App, idx: isize) {
    let results = app.fzf_results.items.len();

    if results > 0 {
        if app.fzf_results.state.selected().is_none() {
            app.fzf_results.state.select(Some(0));
        } else {
            let selected = app.fzf_results.state.selected().unwrap() as isize;
            let new_selected = (selected + idx).rem_euclid(results as isize) as usize;

            app.fzf_results.state.select(Some(new_selected));
        }
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
    if let Some(selected) = app.files.state.selected() {
        let file = app.files.items[selected].0.clone();

        trash::delete(&file).unwrap();
        app.update_files();

        if selected >= app.files.items.len() {
            app.files
                .state
                .select(Some(app.files.items.len().saturating_sub(1)));
        }
    } else if let Some(selected) = app.dirs.state.selected() {
        let dir = app.dirs.items[selected].0.clone();

        trash::delete(&dir).unwrap();
        app.update_dirs();

        if selected >= app.dirs.items.len() {
            app.dirs
                .state
                .select(Some(app.dirs.items.len().saturating_sub(1)));
        }
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
            let dist = levenshtein(&query, &filename);

            if filename == query || dist < 5 {
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

    let result = fzf(app, input);

    app.fzf_results = StatefulList::with_items(
        result
            .iter()
            .map(|x| x.to_str().unwrap().to_string())
            .collect(),
    );
}

fn handle_open_fzf_result(app: &mut App, input: &mut String, input_active: &mut bool) {
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
