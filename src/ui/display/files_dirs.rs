use super::pane::get_pwd;
use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render_files<B: Backend>(f: &mut Frame<B>, app: &mut App, chunks: &[Rect]) {
    let files_block = Block::default()
        .borders(Borders::ALL)
        .title("Files")
        .title_alignment(Alignment::Center);
    f.render_widget(files_block, chunks[0]);

    app.update_files();

    let files = app
        .files
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect::<Vec<ListItem>>();

    let items = List::new(files)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Files")
                .title_alignment(Alignment::Center),
        )
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );

    if app.files.items.len() == 0 {
        let empty = vec![ListItem::new("No files in this directory")];
        let empty_list = List::new(empty)
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_stateful_widget(empty_list, chunks[0], &mut app.files.state);
        return;
    }

    f.render_stateful_widget(items, chunks[0], &mut app.files.state);

    if app.files.state.selected().is_some() {
        let files_block = Block::default()
            .borders(Borders::ALL)
            .title("Files")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::LightBlue));
        f.render_widget(files_block, chunks[0]);
    } else {
        let files_block = Block::default()
            .borders(Borders::ALL)
            .title("Files")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(files_block, chunks[0]);
    }
}

pub fn render_dirs<B: Backend>(f: &mut Frame<B>, app: &mut App, chunks: &[Rect]) {
    app.cur_dir = get_pwd();

    let dirs_block = Block::default()
        .borders(Borders::ALL)
        .title("Directories")
        .title_alignment(Alignment::Center);
    f.render_widget(dirs_block, chunks[0]);

    let dirs = app
        .dirs
        .items
        .iter()
        .map(|i| ListItem::new(i.0.clone()))
        .collect::<Vec<ListItem>>();

    app.update_dirs();

    let items = List::new(dirs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Directories")
                .title_alignment(Alignment::Center),
        )
        .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(items, chunks[0], &mut app.dirs.state);

    if app.dirs.state.selected().is_some() {
        let dirs_block = Block::default()
            .borders(Borders::ALL)
            .title("Directories")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::LightBlue));
        f.render_widget(dirs_block, chunks[0]);
    } else {
        let dirs_block = Block::default()
            .borders(Borders::ALL)
            .title("Directories")
            .title_alignment(Alignment::Center)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(dirs_block, chunks[0]);
    }
}
