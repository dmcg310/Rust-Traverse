use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::widgets::Paragraph;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn render_contents<B: Backend>(f: &mut Frame<B>, app: &mut App, chunks: &[Rect]) {
    let contents_block = Block::default().borders(Borders::ALL).title("Contents");
    f.render_widget(contents_block, chunks[0]);

    let selected_file = match app.files.state.selected() {
        Some(i) => match app.files.items.get(i) {
            Some(item) => &item.0,
            None => "",
        },
        None => "",
    };

    let mut content = String::new();
    let mut total_line_count = 0;

    if !selected_file.is_empty() {
        let file = File::open(selected_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut line = String::new();

        while buf_reader.read_line(&mut line).unwrap() > 0 {
            total_line_count += 1;

            if total_line_count <= 30 {
                content.push_str(&line);
            }

            line.clear();
        }
    }

    if total_line_count > 30 {
        content.push_str(&format!("\n... {} more lines", total_line_count - 30));
        content.push_str(&format!("\n{} total", total_line_count));
    };

    let items = List::new(vec![ListItem::new(content)])
        .block(Block::default().borders(Borders::ALL).title("Contents"));

    f.render_stateful_widget(items, chunks[0], &mut app.files.state);

    if selected_file.is_empty() {
        let placeholder = Paragraph::new("No file selected")
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL).title("Contents"));
        f.render_widget(placeholder, chunks[0]);
    }
}
