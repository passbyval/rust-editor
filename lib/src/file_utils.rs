use std::{
    fs::{self, DirEntry},
    path::Path,
};

use rfd::FileDialog;

use crate::{file_store, State};

pub fn map_paths(path: &Path) -> Vec<DirEntry> {
    fs::read_dir(path).unwrap().map(|p| p.unwrap()).collect()
}

pub fn open_file(state: &mut State, directory: &str) {
    let file = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .add_filter("js", &["js", "jsx", "tsx", "ts", "cjs"])
        .set_directory(directory)
        .pick_file();

    if file.is_some() {
        let path_buf = file.unwrap();
        let file_path = file_store::FileStore::get_file_path(&path_buf);

        state.file_store.insert(&file_path, true);
    }
}
