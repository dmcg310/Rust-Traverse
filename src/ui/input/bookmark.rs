use super::run_app::Command;
use crate::app::app::App;
use dirs::config_dir;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn handle_bookmark(app: &mut App) {
    if app.last_command != Some(Command::Bookmark) {
        read_bookmark(app);
        app.show_bookmark = true;
        app.last_command = Some(Command::Bookmark);
    }
}

pub fn read_bookmark(app: &mut App) {
    if !config_dir()
        .unwrap()
        .join("traverse/bookmarks.txt")
        .exists()
    {
        return;
    }

    let file = std::fs::File::open(config_dir().unwrap().join("traverse/bookmarks.txt")).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        if app.bookmarked_dirs.items.contains(&line) {
            continue;
        } else {
            app.bookmarked_dirs.items.push(line);
        }
    }

    if app.bookmarked_dirs.items.len() > 0 {
        app.bookmarked_dirs.state.select(Some(0));
    }

    app.bookmarked_dirs.items.sort();
}

pub fn add_bookmark(app: &mut App) {
    let path = std::env::current_dir().unwrap();
    let dirs = app.bookmarked_dirs.items.clone();

    if dirs.contains(&path.to_str().unwrap().to_string()) {
        return;
    } else {
        app.bookmarked_dirs
            .items
            .push(path.to_str().unwrap().to_string());

        let mut data = path.to_str().unwrap().to_string();
        data = format!("{}\n", data);

        if !config_dir()
            .unwrap()
            .join("traverse/bookmarks.txt")
            .exists()
        {
            std::fs::create_dir_all(config_dir().unwrap().join("traverse")).unwrap();
            std::fs::File::create(config_dir().unwrap().join("traverse/bookmarks.txt")).unwrap();
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(config_dir().unwrap().join("traverse/bookmarks.txt"))
            .expect("Unable to open file");

        file.write_all(data.as_bytes())
            .expect("Unable to write data");
    }

    if app.bookmarked_dirs.items.len() > 0 {
        app.bookmarked_dirs.state.select(Some(0));
    }

    app.update_bookmarks();
}

pub fn delete_bookmark(app: &mut App) {
    let index = app.bookmarked_dirs.state.selected().unwrap();
    let path = std::env::current_dir().unwrap();
    let dirs = app.bookmarked_dirs.items.clone();

    if dirs.contains(&path.to_str().unwrap().to_string()) {
        app.bookmarked_dirs.items.remove(index);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_dir().unwrap().join("traverse/bookmarks.txt"))
            .expect("Unable to open file");

        for dir in &app.bookmarked_dirs.items {
            let mut data = dir.to_string();
            data = format!("{}\n", data);

            file.write_all(data.as_bytes())
                .expect("Unable to write data");
        }

        file.sync_all().expect("Unable to sync data");
    }

    app.update_bookmarks();
}
