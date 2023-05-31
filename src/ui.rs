use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, time::Duration};

enum PaneState {
    Selected,
    NotSelected,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

struct SelectedPane<T> {
    state: PaneState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }
}

pub struct App {
    files: StatefulList<(&'static str, &'static str)>,
    dirs: StatefulList<(&'static str, &'static str)>,
}

impl App {
    fn new() -> App {
        App {
            files: StatefulList::with_items(vec![
                ("main.rs", "src/main.rs"),
                ("ui.rs", "src/ui.rs"),
                ("Cargo.toml", "Cargo.toml"),
                ("Cargo.lock", "Cargo.lock"),
            ]),
            dirs: StatefulList::with_items(vec![
                ("src", "src/"),
                ("target", "target/"),
                (".git", ".git/"),
            ]),
        }
    }
}

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
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    let dirs_block = Block::default().borders(Borders::ALL).title("Selected");

    f.render_widget(dirs_block, chunks[1]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let files_block = Block::default().borders(Borders::ALL).title("Files");

    f.render_widget(files_block, chunks[0]);

    let dirs_block = Block::default().borders(Borders::ALL).title("Directories");

    f.render_widget(dirs_block, chunks[1]);

    let files: Vec<ListItem> = app.files.items.iter().map(|i| ListItem::new(i.0)).collect();
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

    let dirs: Vec<ListItem> = app.dirs.items.iter().map(|i| ListItem::new(i.0)).collect();
    let selected_dirs = dirs.clone();
    let items = List::new(dirs)
        .block(Block::default().borders(Borders::ALL).title("Directories"))
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(items, chunks[1], &mut app.dirs.state);

    if app.files.state.selected().is_some() {
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
