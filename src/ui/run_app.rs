use super::render::render;
use crate::app::App;
use crate::ui::pane::get_pwd;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::backend::Backend;
use ratatui::terminal::Terminal;
use std::time::Duration;

#[derive(PartialEq)]
pub enum Command {
    CreateFile,
    CreateDir,
    RenameFile,
    RenameDir,
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
                            app.files.state.select(Some(0));
                            app.dirs.state.select(None);
                            app.content.state.select(None);
                        }
                        KeyCode::Char('2') => {
                            app.dirs.state.select(Some(0));
                            app.files.state.select(None);
                            app.content.state.select(None);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if app.files.state.selected().is_some() {
                                app.files.next();
                            } else if app.dirs.state.selected().is_some() {
                                app.dirs.next();
                            } else if app.content.state.selected().is_some() {
                                app.content.next();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if app.files.state.selected().is_some() {
                                app.files.previous();
                            } else if app.dirs.state.selected().is_some() {
                                app.dirs.previous();
                            }
                        }
                        KeyCode::Char('n') => {
                            if app.files.state.selected().is_some() {
                                if (input_active == false
                                    && app.last_command != Some(Command::CreateFile))
                                    || (input_active == true && app.last_command.is_none())
                                {
                                    input_active = true;
                                    app.show_popup = true;
                                    app.last_command = Some(Command::CreateFile);
                                } else {
                                    input.push('n');
                                }
                            } else if app.dirs.state.selected().is_some() {
                                if (input_active == false
                                    && app.last_command != Some(Command::CreateDir))
                                    || (input_active == true && app.last_command.is_none())
                                {
                                    input_active = true;
                                    app.show_popup = true;
                                    app.last_command = Some(Command::CreateDir);
                                } else {
                                    input.push('n');
                                }
                            }
                        }
                        KeyCode::Char('d')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if app.files.state.selected().is_some() {
                                let file = app.files.items[app.files.state.selected().unwrap()]
                                    .0
                                    .clone();

                                std::fs::remove_file(file).unwrap();
                                app.update_files();
                            } else if app.dirs.state.selected().is_some() {
                                let dir =
                                    app.dirs.items[app.dirs.state.selected().unwrap()].0.clone();

                                std::fs::remove_dir_all(dir).unwrap();
                                app.update_dirs();
                            }
                        }
                        KeyCode::Char('r') => {
                            if app.files.state.selected().is_some() {
                                if (input_active == false
                                    && app.last_command != Some(Command::RenameFile))
                                    || (input_active == true && app.last_command.is_none())
                                {
                                    input_active = true;
                                    app.show_popup = true;
                                    app.last_command = Some(Command::RenameFile);
                                    input = app.files.items[app.files.state.selected().unwrap()]
                                        .0
                                        .clone();
                                } else {
                                    input.push('r');
                                }
                            } else if app.dirs.state.selected().is_some() {
                                if (input_active == false
                                    && app.last_command != Some(Command::RenameDir))
                                    || (input_active == true && app.last_command.is_none())
                                {
                                    input_active = true;
                                    app.show_popup = true;
                                    app.last_command = Some(Command::RenameDir);
                                    input = app.dirs.items[app.dirs.state.selected().unwrap()]
                                        .0
                                        .clone();
                                } else {
                                    input.push('r');
                                }
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if input_active {
                                input_active = false;
                                app.show_popup = false;
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Char(c) => {
                            if input_active {
                                input.push(c);
                            }
                        }
                        KeyCode::Enter => {
                            if input_active {
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
                                    let dir = app.dirs.items[app.dirs.state.selected().unwrap()]
                                        .0
                                        .clone();

                                    std::fs::rename(dir, input.clone()).unwrap();
                                    app.update_dirs();
                                    app.last_command = None;
                                }
                                input.clear();
                                app.show_popup = false;
                                input_active = false;
                            } else {
                                if app.dirs.state.selected().is_some() {
                                    if app.dirs.items[app.dirs.state.selected().unwrap()].0 == "../"
                                    {
                                        let mut path = std::env::current_dir().unwrap();
                                        path.pop();

                                        std::env::set_current_dir(path).unwrap();
                                        app.cur_dir = get_pwd();
                                    } else {
                                        let dir = app.dirs.items
                                            [app.dirs.state.selected().unwrap()]
                                        .0
                                        .clone();

                                        std::env::set_current_dir(dir).unwrap();
                                        app.cur_dir = get_pwd();
                                    }
                                    app.update_files();
                                    app.update_dirs();

                                    if let Some(selected) = app.files.state.selected() {
                                        if selected >= app.files.items.len() {
                                            if !app.files.items.is_empty() {
                                                app.files.state.select(Some(
                                                    app.files.items.len().saturating_sub(1),
                                                ));
                                            } else {
                                                app.files.state.select(None);
                                            }
                                        }
                                    }
                                    app.dirs.state.select(Some(0));
                                }
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
