use crate::app::app::App;
use flate2::read::GzDecoder;
use std::io::Read;
use std::{fs::File, io::Cursor};
use tar::Archive;

pub fn extract_tar(app: &mut App, file: &str) -> Result<(), std::io::Error> {
    let path = std::env::current_dir().unwrap().join(file);

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    app.update_files();
    app.update_dirs();

    Ok(())
}

pub fn extract_zip(app: &mut App, file: &str) -> Result<(), std::io::Error> {
    let target_dir = std::env::current_dir().unwrap();

    let mut file = File::open(file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let reader = Cursor::new(buffer);

    zip_extract::extract(reader, &target_dir, true).unwrap();

    app.update_files();
    app.update_dirs();

    Ok(())
}
