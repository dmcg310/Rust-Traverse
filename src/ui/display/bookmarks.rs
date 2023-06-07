use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::Clear;
use ratatui::widgets::ListItem;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List},
    Frame,
};
pub fn render_bookmark<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect) {
    if app.show_bookmark {
        let block_width = f.size().width / 3;
        let block_height = f.size().height / 3;
        let block_x = (size.width - block_width) / 2;
        let block_y = (size.height - block_height) / 2;

        let area = Rect::new(block_x, block_y, block_width, block_height);

        let bookmark_block = Block::default()
            .style(Style::default().add_modifier(Modifier::BOLD))
            .title("Bookmarks")
            .border_style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(bookmark_block, area);

        let bookmark_text = app
            .bookmarked_dirs
            .items
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect::<Vec<ListItem>>();

        let bookmark_list = List::new(bookmark_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Bookmarks")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            )
            .highlight_symbol("> ");

        let bookmark_list_area =
            Rect::new(block_x + 1, block_y + 1, block_width - 2, block_height - 2);

        f.render_stateful_widget(
            bookmark_list,
            bookmark_list_area,
            &mut app.bookmarked_dirs.state,
        );
    }
}
