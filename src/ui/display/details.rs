use crate::app::app::App;
use crate::ui::display::pane::selected_pane_content;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::Paragraph;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render_details<B: Backend>(
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

    // to fit the path in the pane
    let mut cur_dir = cur_dir;
    let components: Vec<&str> = cur_dir.split("/").collect();
    if components.len() > 4 {
        let last_three: Vec<&str> = components.into_iter().rev().take(3).collect();
        cur_dir = format!(
            ".../{}",
            last_three
                .into_iter()
                .rev()
                .collect::<Vec<&str>>()
                .join("/")
        );
    }

    let selected_file = match app.files.state.selected() {
        Some(i) => match app.files.items.get(i) {
            Some(item) => &item.0,
            None => "",
        },
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

    let items = List::new(selected_item).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightYellow))
            .title("Details")
            .title_alignment(Alignment::Left),
    );
    f.render_widget(items, details_chunks[0]);

    let pwd_paragraph = Paragraph::new(cur_dir)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightYellow))
                .title_alignment(Alignment::Center)
                .title("Current Directory"),
        )
        .alignment(Alignment::Center);
    f.render_widget(pwd_paragraph, details_chunks[1]);

    let du_paragraph = Paragraph::new(cur_du)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightYellow))
                .title("Disk Usage")
                .title_alignment(Alignment::Right),
        )
        .alignment(Alignment::Right);
    f.render_widget(du_paragraph, details_chunks[2]);
}
