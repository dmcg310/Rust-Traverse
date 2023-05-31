use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
use std::{io, thread, time::Duration};

fn main() -> Result<(), io::Error> {
    let files = ["foo.txt", "bar.rs", "baz.js"];
    let dirs = ["foo", "bar", "baz"];

    enable_raw_mode()?;
    let stdout = io::stdout();

    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture,)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
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
