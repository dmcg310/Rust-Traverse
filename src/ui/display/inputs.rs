use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

pub fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect, input: &mut String) {
    if app.show_popup {
        let block = Block::default()
            .title("Name")
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
            .block(
                Block::default()
                    .title("Input")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightBlue)),
            )
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Left);
        f.render_widget(input_box, area);
    }
}
