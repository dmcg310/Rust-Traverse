use crate::{app::app::App, ui::display::block::block_binds};

pub fn handle_movement(app: &mut App, key: char) {
    if block_binds(app) {
        return;
    }

    if app.files.state.selected().is_some() {
        if app.files.items.len() > 1 {
            if key == 'j' {
                app.files.next();
            } else {
                app.files.previous();
            }
        }
    } else if app.dirs.state.selected().is_some() {
        if app.dirs.items.len() > 1 {
            if key == 'j' {
                app.dirs.next();
            } else {
                app.dirs.previous();
            }
        }
    }
}

pub fn handle_fzf_movement(app: &mut App, idx: isize) {
    let results = app.fzf_results.items.len();

    if results > 0 {
        if app.fzf_results.state.selected().is_none() {
            app.fzf_results.state.select(Some(0));
        } else {
            let selected = app.fzf_results.state.selected().unwrap() as isize;
            let new_selected = (selected + idx).rem_euclid(results as isize) as usize;

            app.fzf_results.state.select(Some(new_selected));
        }
    }
}

pub fn handle_bookmark_movement(app: &mut App, idx: isize) {
    let results = app.bookmarked_dirs.items.len();

    if results > 0 {
        if app.bookmarked_dirs.state.selected().is_none() {
            app.bookmarked_dirs.state.select(Some(0));
        } else {
            let selected = app.bookmarked_dirs.state.selected().unwrap() as isize;
            let new_selected = (selected + idx).rem_euclid(results as isize) as usize;

            app.bookmarked_dirs.state.select(Some(new_selected));
        }
    }
}

pub fn handle_pane_switching(app: &mut App, key: u8) {
    if block_binds(app) {
        return;
    }

    if key == 1 {
        app.files.state.select(Some(0));
        app.dirs.state.select(None);
    } else if key == 2 {
        app.dirs.state.select(Some(0));
        app.files.state.select(None);
    }
}

pub fn handle_ops_menu_movement(app: &mut App, idx: isize) {
    let results = app.ops_menu.items.len();

    if results > 0 {
        if app.ops_menu.state.selected().is_none() {
            app.ops_menu.state.select(Some(0));
        } else {
            let selected = app.ops_menu.state.selected().unwrap() as isize;
            let new_selected = (selected + idx).rem_euclid(results as isize) as usize;

            app.ops_menu.state.select(Some(new_selected));
        }
    }
}
