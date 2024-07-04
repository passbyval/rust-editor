use std::{fs::DirEntry, path::PathBuf};

use egui::{CursorIcon, Ui};

use crate::{file_list, file_utils};

pub fn create(ui: &mut Ui, paths: &mut Vec<DirEntry>, mut files: &mut file_list::FileList) {
    paths.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .is_dir()
            .cmp(&a.metadata().unwrap().is_dir())
    });

    for path in &mut *paths {
        let file_name = path.file_name().into_string().unwrap();

        if path.metadata().unwrap().is_dir() {
            let path_buff: PathBuf = path.path();
            let mut new_paths = file_utils::map_paths(&path_buff);

            let folder = egui::CollapsingHeader::new(file_name);

            folder
                .show(ui, |inner_ui| {
                    create(inner_ui, &mut new_paths, &mut files);
                })
                .header_response
                .on_hover_cursor(CursorIcon::PointingHand);
        } else {
            let label = ui
                .selectable_label(false, file_name)
                .on_hover_cursor(CursorIcon::PointingHand);

            if label.clicked() {
                let path_buf = path.path();
                let file_path = file_list::FileList::get_file_path(&path_buf);

                files.insert(&file_path, true);
            }
        }
    }
}
