use super::*;
use crate::app::app::App;
use crate::ui::display::block::block_binds;
use crate::ui::display::render::render;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::backend::Backend;
use ratatui::terminal::Terminal;
use std::process::Command as SysCommand;
use std::time::Duration;

#[derive(PartialEq)]
pub enum Command {
    CreateFile,
    CreateDir,
    RenameFile,
    RenameDir,
    ShowNav,
    ShowFzf,
    ShowHelp,
    Bookmark,
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
                        // EXIT
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            SysCommand::new("reset").status().unwrap_or_else(|_| {
                                panic!("Failed to reset terminal");
                            });
                            nav::output_cur_dir();

                            return Ok(());
                        }
                        KeyCode::Esc => {
                            if app.show_popup
                                || app.show_nav
                                || app.show_fzf
                                || app.show_bookmark
                                || app.show_help
                                || app.show_ops_menu
                            {
                                input_active = false;
                                app.show_popup = false;
                                app.show_nav = false;
                                app.show_fzf = false;
                                app.last_command = None;
                                app.show_bookmark = false;
                                app.show_help = false;
                                app.show_ops_menu = false;
                                input.clear();
                            } else {
                                SysCommand::new("reset").status().unwrap_or_else(|_| {
                                    panic!("Failed to reset terminal");
                                });
                                nav::output_cur_dir();

                                return Ok(());
                            }
                        }
                        KeyCode::Char('q') => {
                            if app.show_fzf || app.show_nav || input_active {
                                input.push('q');
                            } else {
                                if app.show_popup
                                    || app.show_nav
                                    || app.show_fzf
                                    || app.show_bookmark
                                    || app.show_help
                                    || app.show_ops_menu
                                {
                                    input_active = false;
                                    app.show_popup = false;
                                    app.show_nav = false;
                                    app.show_fzf = false;
                                    app.last_command = None;
                                    app.show_bookmark = false;
                                    app.show_help = false;
                                    app.show_ops_menu = false;
                                    input.clear();
                                } else {
                                    SysCommand::new("reset").status().unwrap_or_else(|_| {
                                        panic!("Failed to reset terminal");
                                    });

                                    nav::output_cur_dir();
                                    return Ok(());
                                }
                            }
                        }

                        // PANE SWITCHING
                        KeyCode::Char('1') => {
                            if input_active {
                                input.push('1');
                            } else {
                                movement::handle_pane_switching(&mut app, 1);
                            }
                        }
                        KeyCode::Char('2') => {
                            if input_active {
                                input.push('2');
                            } else {
                                movement::handle_pane_switching(&mut app, 2);
                            }
                        }

                        // MOVEMENT
                        KeyCode::Char('j') | KeyCode::Down => {
                            if input_active {
                                input.push('j');
                            } else {
                                movement::handle_movement(&mut app, 'j');
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if input_active {
                                input.push('k');
                            } else {
                                movement::handle_movement(&mut app, 'k');
                            }
                        }
                        KeyCode::Char('n')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if app.show_fzf && block_binds(&mut app) {
                                movement::handle_fzf_movement(&mut app, 1);
                            } else if app.show_bookmark {
                                movement::handle_bookmark_movement(&mut app, 1);
                            } else if app.show_ops_menu {
                                movement::handle_ops_menu_movement(&mut app, 1);
                            }
                        }
                        KeyCode::Char('p')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if app.show_fzf && block_binds(&mut app) {
                                movement::handle_fzf_movement(&mut app, -1);
                            } else if app.show_bookmark {
                                movement::handle_bookmark_movement(&mut app, -1);
                            } else if app.show_ops_menu {
                                movement::handle_ops_menu_movement(&mut app, -1);
                            }
                        }

                        // BOOKMARKS
                        KeyCode::Char('z') => {
                            if input_active {
                                input.push('z');
                            } else {
                                bookmark::add_bookmark(&mut app);
                            }
                        }
                        KeyCode::Char('b') => {
                            if input_active {
                                input.push('b');
                            } else {
                                bookmark::handle_bookmark(&mut app);
                            }
                        }

                        // FILE OPS
                        KeyCode::Char('n') => {
                            if input_active {
                                input.push('n');
                            } else {
                                file_ops::handle_new_file(&mut app, &mut input_active);
                            }
                        }
                        KeyCode::Char('d')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            if app.show_bookmark {
                                bookmark::delete_bookmark(&mut app);
                            } else {
                                file_ops::handle_delete(&mut app);
                            }
                        }
                        KeyCode::Char('c') => {
                            if input_active {
                                input.push('c');
                            } else {
                                file_ops::add_to_selected(&mut app);
                            }
                        }
                        KeyCode::Char('p') => {
                            if input_active {
                                input.push('p');
                            } else {
                                if app.files.state.selected().is_some()
                                    || app.dirs.state.selected().is_some()
                                {
                                    app.show_ops_menu = true;
                                }
                            }
                        }
                        KeyCode::Char('x') => {
                            if input_active {
                                input.push('x');
                            } else {
                                file_ops::extract(&mut app);
                            }
                        }
                        KeyCode::Char('r') => {
                            if input_active {
                                input.push('r');
                            } else {
                                file_ops::handle_rename(&mut app, &mut input, &mut input_active);
                            }
                        }

                        // HELP MENU
                        KeyCode::Char('?') => {
                            if input_active {
                                input.push('?');
                            } else if app.show_help {
                                app.show_help = false;
                                app.last_command = None;
                            } else {
                                help::handle_help(&mut app);
                            }
                        }

                        // FZF & NAV
                        KeyCode::Char('w') => {
                            if input_active {
                                input.push('w');
                            } else {
                                nav::handle_fzf(&mut app, &mut input, &mut input_active);
                            }
                        }
                        KeyCode::Char('f') => {
                            if input_active {
                                input.push('f');
                            } else {
                                nav::handle_nav(&mut app, &mut input_active);
                            }
                        }

                        // SUBMIT
                        KeyCode::Enter => {
                            if app.show_fzf {
                                submit::handle_open_fzf_result(
                                    &mut app,
                                    &mut input,
                                    &mut input_active,
                                );
                            } else if input_active {
                                submit::handle_submit(&mut app, &mut input, &mut input_active);
                            } else if app.show_bookmark {
                                submit::handle_open_bookmark(&mut app);
                            } else if app.show_ops_menu {
                                if app.ops_menu.state.selected().is_none() {
                                    app.show_ops_menu = false;
                                    app.last_command = None;
                                } else {
                                    file_ops::handle_paste_or_move(&mut app);
                                }
                            } else {
                                submit::handle_submit(&mut app, &mut input, &mut input_active);
                            }
                        }

                        // BACKSPACE
                        KeyCode::Backspace => {
                            if input_active {
                                input.pop();
                                if app.show_fzf {
                                    nav::handle_fzf(&mut app, &mut input, &mut input_active);
                                }
                            }
                        }

                        // OTHER CHARACTERS
                        KeyCode::Char(c) => {
                            if input_active {
                                input.push(c);

                                if app.last_command == Some(Command::ShowFzf) {
                                    nav::handle_fzf(&mut app, &mut input, &mut input_active);
                                }

                                if app.show_fzf {
                                    nav::handle_fzf(&mut app, &mut input, &mut input_active);
                                }
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
