use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

pub(crate) fn find_wacom() -> io::Result<PathBuf> {
    find_file("/dev/input/by-path/", "event-mouse")
}

pub(crate) fn find_keyboard() -> io::Result<PathBuf> {
    find_file("/dev/input/by-path/", "event-kbd")
}

fn find_file(path: &str, name: &str) -> io::Result<PathBuf> {
    let dir = fs::read_dir(path)?;
    for file in dir {
        let file = file?;

        if let Some(file_name) = file.file_name().to_str() {
            if file_name.contains(name) {
                return Ok(file.path().clone());
            }
        }
    }
    Err(io::Error::new(ErrorKind::NotFound, "File not found"))
}
