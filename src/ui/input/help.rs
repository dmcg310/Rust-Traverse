use super::*;
use crate::app::app::App;
use run_app::Command;

pub fn handle_help(app: &mut App) {
    if app.last_command != Some(Command::ShowHelp) {
        app.show_help = true;
        app.last_command = Some(Command::ShowHelp);
    }
}
