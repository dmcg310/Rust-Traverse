use crate::app::app::App;
use crate::ui::display::*;
use crate::ui::input::run_app::run_app;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Terminal,
    Frame,
};
use std::io;
use std::time::Duration;

pub fn init() -> Result<()> {
    enable_raw_mode()?;

    let stdout = io::stdout();
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture,)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();
    app.op_menu_init();
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
    let fifty_percent = (size.width as f32 * 0.5) as u16;
    let ninety_percent = (size.height as f32 * 0.9) as u16;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(fifty_percent), Constraint::Min(1)])
        .split(size);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(ninety_percent), Constraint::Min(1)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(45),
            Constraint::Percentage(10),
        ])
        .split(chunks[1]);

    let bottom_chunks = bottom_chunks(f);

    contents::render_contents(f, app, &left_chunks);
    files_dirs::render_files(f, app, &[right_chunks[0]]);
    files_dirs::render_dirs(f, app, &[right_chunks[1]]);
    details::render_details(f, app, &bottom_chunks, cur_dir, cur_du);
    inputs::render_input(f, app, size, input);
    navs::render_navigator(f, app, size, input);
    navs::render_fzf(f, app, size);
    help::render_help(f, app, size);
    bookmarks::render_bookmark(f, app, size);
    ops::render_ops_menu(f, app, size);
}

fn bottom_chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let size = f.size();
    let ninety_percent = (size.height as f32 * 0.9) as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(ninety_percent), Constraint::Min(1)])
        .split(size);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(f.size().width / 2)])
        .split(chunks[1]);

    (bottom_chunks).to_vec()
}
