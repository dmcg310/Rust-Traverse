use crate::app::app::App;
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

pub fn render_help<B: Backend>(f: &mut Frame<B>, app: &mut App, size: Rect) {
    if app.show_help {
        let block_width = f.size().width / 2;
        let block_height = f.size().height;
        let block_x = (size.width - block_width) / 2;
        let block_y = (size.height - block_height) / 2;

        let area = Rect::new(block_x, block_y, block_width, block_height);

        let help_block = Block::default()
            .style(Style::default().add_modifier(Modifier::BOLD))
            .border_style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center);

        f.render_widget(Clear, area);
        f.render_widget(help_block, area);

        let mut help_text = String::new();
        // formatted like this because tui rs doesn't render it nicely
        help_text.push_str(
            "Traverse 2023
ESC | q: Quit the application.
1: Select the Files pane.
2: Select the Directories pane.

j: Select the next item in the current pane.
k: Select the previous item in the current pane.

n: Create a new file or directory, depending on the current pane.
CTRL + d: Delete the selected file or directory, (to bin).
r: Rename the selected file or directory.

f: Navigate to a directory using a relative or absolute path.
x: Extract the selected archive, to the current directory.
w: Open fzf.

c: Append the selected file or directory to the move/copy buffer.
p: Opens the move/copy buffer menu, (enter on any option is in 
            relation to your current directory).

b: Shows bookmarks menu.
z: Add current directory to bookmarks.

CTRL + n: 'Next' item in results.
CTRL + p: 'Previous' item in results.",
        );

        let help_para = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Bindings")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);

        f.render_widget(help_para, area);
    }
}
