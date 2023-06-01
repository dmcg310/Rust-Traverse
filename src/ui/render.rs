use crate::app::App;
use crate::ui::pane::{get_pwd, selected_pane_content};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::Paragraph;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::Terminal,
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use std::io;
use std::time::Duration;

pub fn render() -> Result<()> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture,)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("{}", e);
    }

    Ok(())
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

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
                        }
                        KeyCode::Char('2') => {
                            app.dirs.state.select(Some(0));
                            app.files.state.select(None);
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if app.files.state.selected().is_some() {
                                app.files.next();
                            } else if app.dirs.state.selected().is_some() {
                                app.dirs.next();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if app.files.state.selected().is_some() {
                                app.files.previous();
                            } else if app.dirs.state.selected().is_some() {
                                app.dirs.previous();
                            }
                        }
                        KeyCode::Enter => {
                            if app.dirs.state.selected().is_some() {
                                if app.dirs.items[app.dirs.state.selected().unwrap()].0 == "../" {
                                    let mut path = std::env::current_dir().unwrap();
                                    path.pop();

                                    std::env::set_current_dir(path).unwrap();
                                    app.cur_dir = get_pwd();
                                } else {
                                    let dir = app.dirs.items[app.dirs.state.selected().unwrap()]
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
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(94), Constraint::Percentage(6)].as_ref())
        .split(size);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let selected_block = Block::default().borders(Borders::ALL);
    f.render_widget(selected_block, bottom_chunks[0]);

    let current_dir_paragraph = Paragraph::new(app.cur_dir.clone())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(current_dir_paragraph, bottom_chunks[1]);

    let disk_paragraph = Paragraph::new(app.cur_du.clone())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(disk_paragraph, bottom_chunks[2]);

    let binding = "".to_string();
    let selected_file = match app.files.state.selected() {
        Some(i) => &app.files.items[i].0,
        None => &binding,
    };

    let binding = "".to_string();
    let selected_dir = match app.dirs.state.selected() {
        Some(i) => &app.dirs.items[i].0,
        None => &binding,
    };

    let selected_item = if !selected_file.is_empty() {
        selected_pane_content(selected_file)
    } else if !selected_dir.is_empty() {
        selected_pane_content(selected_dir)
    } else {
        vec![ListItem::new(Spans::from("No file selected"))]
    };

    let items = List::new(selected_item)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(items, chunks[1], &mut app.files.state);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let files_block = Block::default().borders(Borders::ALL).title("Files");
    f.render_widget(files_block, chunks[0]);

    let dirs_block = Block::default().borders(Borders::ALL).title("Directories");
    f.render_widget(dirs_block, chunks[1]);

    let files: Vec<ListItem> = app
        .files
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect();
    let selected_files = files.clone();

    let items = List::new(files)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(items, chunks[0], &mut app.files.state);

    let dirs: Vec<ListItem> = app
        .dirs
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect();

    let selected_dirs = dirs.clone();

    let items = List::new(dirs)
        .block(Block::default().borders(Borders::ALL).title("Directories"))
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(items, chunks[1], &mut app.dirs.state);

    if app.files.state.selected().is_some() {
        // to rerender pane
        let files_block = Block::default().borders(Borders::ALL).title("Files");
        f.render_widget(files_block, chunks[0]);

        let files_block = List::new(selected_files)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Files")
                    .border_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(files_block, chunks[0], &mut app.files.state);
    } else if app.dirs.state.selected().is_some() {
        // to rerender pane
        let dirs_block = Block::default().borders(Borders::ALL).title("Files");
        f.render_widget(dirs_block, chunks[0]);

        let dirs_block = List::new(selected_dirs)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Directories")
                    .border_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(dirs_block, chunks[1], &mut app.dirs.state);
    }
}
