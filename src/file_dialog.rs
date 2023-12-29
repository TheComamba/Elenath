use std::path::PathBuf;

fn current_path() -> PathBuf {
    match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => PathBuf::default(),
    }
}

pub(crate) fn new() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_file_name("celestial_system.json")
        .set_directory(current_path())
        .save_file()
}

pub(crate) fn open() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Celestial System (.json)", &["json"])
        .add_filter("Any", &["*"])
        .set_directory(current_path())
        .pick_file()
}
