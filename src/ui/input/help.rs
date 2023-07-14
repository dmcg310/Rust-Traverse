use super::*;
use crate::app::app::App;
use crate::ui::display::block::block_binds;
use run_app::Command;

pub fn handle_help(app: &mut App) {
    if block_binds(app) {
        return;
    }

    if app.last_command != Some(Command::ShowHelp) {
        app.show_help = true;
        app.last_command = Some(Command::ShowHelp);
    }
}
