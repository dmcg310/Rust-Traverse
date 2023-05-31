use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::Borders,
    widgets::{Block, List, ListItem, ListState},
    Terminal,
};
use std::{io, thread, time::Duration};

fn main() -> Result<(), io::Error> {
    let files = [ListItem::new("bar.rs"), ListItem::new("baz.js")];

    let dirs = [
        ListItem::new("foo"),
        ListItem::new("bar"),
        ListItem::new("baz"),
    ];

    enable_raw_mode()?;
    let stdout = io::stdout();

    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture,)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
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

        let files = List::new(files)
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .highlight_symbol(">> ")
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        let dirs = List::new(dirs)
            .block(Block::default().borders(Borders::ALL).title("Directories"))
            .highlight_symbol(">> ")
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(files, chunks[0], &mut ListState::default());
        f.render_stateful_widget(dirs, chunks[1], &mut ListState::default());
    })?;

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
    });

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
