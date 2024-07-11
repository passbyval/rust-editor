use crate::{file_store, State};
use rfd::AsyncFileDialog;
use std::{
    fs::{self, DirEntry},
    path::Path,
};

pub fn map_paths(path: &Path) -> Vec<DirEntry> {
    fs::read_dir(path).unwrap().map(|p| p.unwrap()).collect()
}

pub fn open_file(state: &mut State, directory: &str) {
    let _future = async {
        let file = AsyncFileDialog::new()
            .add_filter("text", &["txt"])
            .add_filter("rust", &["rs", "toml"])
            .add_filter("js", &["js", "jsx", "tsx", "ts", "cjs"])
            .set_directory(directory)
            .pick_file()
            .await;

        // let data: Vec<u8> = file.unwrap().read().await;

        if file.is_some() {
            let handle = file.unwrap();
            let path = handle.path();
            let file_path = file_store::FileStore::get_file_path(&path);

            state.file_store.insert(&file_path, true);
        }
    };
}
