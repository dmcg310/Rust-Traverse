use crate::app::app::App;
use crate::ui::input::nav::abbreviate_path;
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

pub fn render_ops_menu<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect) {
    if app.show_ops_menu {
        let block_width = f.size().width / 2;
        let block_height = f.size().height / 3;
        let block_x = (size.width - block_width) / 2;
        let block_y = (size.height - block_height) / 2;

        let area = Rect::new(block_x, block_y, block_width, block_height);
        let half_area = Rect::new(block_x, block_y, block_width / 2, block_height);

        let ops_menu_block = Block::default()
            .style(Style::default().add_modifier(Modifier::BOLD))
            .border_style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .title_alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(ops_menu_block, half_area);

        let ops_text = app
            .ops_menu
            .items
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect::<Vec<ListItem>>();

        let ops_list = List::new(ops_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Operations")
                    .border_style(
                        Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            )
            .highlight_symbol("> ");

        let ops_menu_list_area = Rect::new(
            block_x + 1,
            block_y + 1,
            (block_width / 2) - 2,
            block_height - 2,
        );

        f.render_stateful_widget(ops_list, ops_menu_list_area, &mut app.ops_menu.state);

        let mut selected_files_clone = app.selected_files.clone();

        if selected_files_clone.is_empty() {
            selected_files_clone.push("No files staged for operation".to_string());
        }

        let selected_files_text = selected_files_clone
            .iter()
            .map(|i| ListItem::new(abbreviate_path(i)))
            .collect::<Vec<ListItem>>();

        let selected_files_list = List::new(selected_files_text).block(
            Block::default()
                .style(Style::default().add_modifier(Modifier::BOLD))
                .title("Currently Selected Files/Dirs")
                .border_style(
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        );

        let selected_files_list_area = Rect::new(
            block_x + block_width / 2 + 1,
            block_y + 1,
            block_width / 2 - 2,
            block_height - 2,
        );

        f.render_stateful_widget(
            selected_files_list,
            selected_files_list_area,
            &mut app.ops_menu.state,
        );
    }
}
