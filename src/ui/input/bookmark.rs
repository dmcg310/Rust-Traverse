use super::run_app::Command;
use crate::app::app::App;

pub fn handle_bookmark(app: &mut App) {
    if app.last_command != Some(Command::Bookmark) {
        app.show_bookmark = true;
        app.last_command = Some(Command::Bookmark);
    }
}

pub fn add_bookmark(app: &mut App) {
    // TODO: write to file
    let path = std::env::current_dir().unwrap();
    let dirs = app.bookmarked_dirs.items.clone();

    if dirs.contains(&path.to_str().unwrap().to_string()) {
        return;
    } else {
        app.bookmarked_dirs
            .items
            .push(path.to_str().unwrap().to_string());
    }

    app.update_bookmarks();
}
