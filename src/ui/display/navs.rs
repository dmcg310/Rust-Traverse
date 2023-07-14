use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::ListItem;
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List},
    Frame,
};

pub fn render_navigator<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    size: Rect,
    input: &mut String,
) {
    if app.show_nav {
        let block = Block::default()
            .title("Navigator")
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        let input_box_width = 30;
        let input_box_height = 3;
        let input_box_x = (size.width - input_box_width) / 4 + 3;
        let input_box_y = (size.height - input_box_height) / 1;

        let area = Rect::new(input_box_x, input_box_y, input_box_width, input_box_height);

        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let input_box = Paragraph::new(input.clone())
            .style(Style::default())
            .block(Block::default().title("Navigator").borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left);
        f.render_widget(input_box, area);
    }
}

pub fn render_fzf<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect) {
    if app.show_fzf {
        let block_width = f.size().width / 1;
        let block_height = f.size().height / 2;
        let block_x = (size.width - block_width) / 2;
        let block_y = (size.height - block_height) / 2;

        let area = Rect::new(block_x, block_y, block_width, block_height);

        let results_block = Block::default()
            .style(Style::default().add_modifier(Modifier::BOLD))
            .title("FZF")
            .border_style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(results_block, area);

        let results_text = app
            .fzf_results
            .items
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect::<Vec<ListItem>>();

        let results_list = List::new(results_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Results")
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::LightGreen),
            )
            .highlight_symbol("> ");

        let results_list_area =
            Rect::new(block_x + 1, block_y + 1, block_width - 2, block_height - 2);

        f.render_stateful_widget(results_list, results_list_area, &mut app.fzf_results.state);
    }
}
