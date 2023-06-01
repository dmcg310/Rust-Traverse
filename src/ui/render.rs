use crate::app::{centered_rect, App};
use crate::ui::pane::selected_pane_content;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Terminal,
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use std::io;
use std::time::Duration;

use super::pane::{get_du, get_pwd};
use super::run_app::run_app;

pub fn init() -> Result<()> {
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

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App, input: &mut String) {
    let cur_dir = app.cur_dir.clone();
    let cur_du = app.cur_du.clone();

    let size = f.size();

    let top_chunks = top_chunks(f);
    let bottom_chunks = bottom_chunks(f);

    render_files(f, app, &top_chunks);
    render_dirs(f, app, &top_chunks);
    render_details(f, app, &bottom_chunks, cur_dir, cur_du);
    render_input(f, app, size, input);

    if app.files.state.selected().is_some() {
        render_files(f, app, &top_chunks)
    } else if app.dirs.state.selected().is_some() {
        render_dirs(f, app, &top_chunks)
    }
}

fn top_chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(94), Constraint::Percentage(6)].as_ref())
        .split(size);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    (top_chunks).to_vec()
}

fn bottom_chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(94), Constraint::Percentage(6)].as_ref())
        .split(size);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    (bottom_chunks).to_vec()
}

fn render_files<B: Backend>(f: &mut Frame<B>, app: &mut App, chunks: &[Rect]) {
    let files_block = Block::default().borders(Borders::ALL).title("Files");
    f.render_widget(files_block, chunks[0]);

    let files: Vec<ListItem> = app
        .files
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect();

    let items = List::new(files)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
    f.render_stateful_widget(items, chunks[0], &mut app.files.state);
}

fn render_dirs<B: Backend>(f: &mut Frame<B>, app: &mut App, chunks: &[Rect]) {
    app.cur_dir = get_pwd();
    let dirs_block = Block::default().borders(Borders::ALL).title("Directories");
    f.render_widget(dirs_block, chunks[1]);

    let dirs: Vec<ListItem> = app
        .dirs
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect();

    let items = List::new(dirs)
        .block(Block::default().borders(Borders::ALL).title("Directories"))
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(items, chunks[1], &mut app.dirs.state);
}

fn render_details<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    chunks: &[Rect],
    cur_dir: String,
    cur_du: String,
) {
    let details_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let selected_file = match app.files.state.selected() {
        Some(i) => &app.files.items[i].0,
        None => "",
    };

    let selected_dir = match app.dirs.state.selected() {
        Some(i) => &app.dirs.items[i].0,
        None => "",
    };

    let selected_item = if !selected_file.is_empty() {
        selected_pane_content(&selected_file.to_string())
    } else if !selected_dir.is_empty() {
        selected_pane_content(&selected_dir.to_string())
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
    f.render_widget(items, details_chunks[0]);

    let pwd_paragraph = Paragraph::new(cur_dir)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Directory"),
        )
        .alignment(Alignment::Left);
    f.render_widget(pwd_paragraph, details_chunks[1]);

    let du_paragraph = Paragraph::new(cur_du)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Disk Usage"))
        .alignment(Alignment::Left);
    f.render_widget(du_paragraph, details_chunks[2]);
}

fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect, input: &mut String) {
    if app.show_popup {
        let block = Block::default()
            .title("Name")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        let area = centered_rect(30, 7, size);
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let input_box = Paragraph::new(input.clone())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);
        f.render_widget(input_box, area);
    }
}
