use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::iter::Map;
use std::path::{Ancestors, Path, PathBuf};
use std::{array, iter};

use egui::TextBuffer;

#[derive(Debug, Clone)]
pub struct FileData {
    pub name: String,
    pub content: String,
}

pub struct FileStore {
    pub files: HashMap<String, FileData>,
    pub active_file: String,
    components: Vec<(String, String)>,
}

impl Default for FileStore {
    fn default() -> Self {
        FileStore {
            files: HashMap::new(),
            active_file: "".into(),
            components: Vec::new(),
        }
    }
}

impl FileStore {
    pub fn new() -> Self {
        Self {
            ..FileStore::default()
        }
    }

    pub fn insert(&mut self, file_path: &String, active: bool) {
        let path_buf = PathBuf::from(file_path).to_path_buf();
        let name = FileStore::get_file_name(&path_buf);

        let content = match self.files.get(file_path) {
            Some(FileData { content, .. }) => {
                // TODO: handle file mismatch.
                content.to_string()
            }
            None => {
                let buff = fs::read(&file_path).expect("Should have been able to read the file");
                String::from_utf8_lossy(&buff).to_string()
            }
        };

        if active {
            self.active_file = file_path.to_string();

            self.components = path_buf
                .components()
                .into_iter()
                .map(|a| {
                    let component = a.as_os_str().to_string_lossy().take();

                    let name = if component == "." {
                        match Path::new(&a).canonicalize().unwrap().file_name() {
                            Some(path) => path.to_string_lossy().take(),
                            None => "".into(),
                        }
                    } else {
                        component
                    };

                    (name, Path::new(&a).to_string_lossy().to_string())
                })
                .collect();
        }

        self.files
            .insert(file_path.to_string(), FileData { name, content });
    }

    pub fn get_file_path(path: &Path) -> String {
        path.to_path_buf().into_os_string().into_string().unwrap()
    }

    pub fn get_file_name(path: &Path) -> String {
        path.to_path_buf()
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    pub fn get_active_file_id(&self) -> String {
        self.active_file.to_string()
    }

    pub fn get_active_file_components(&self) -> Vec<(String, String)> {
        self.components.clone()
    }

    pub fn set_active_file(&mut self, file_path: &String) {
        self.active_file = file_path.to_string();
    }

    pub fn get_active_file_as_mut(&mut self) -> Option<&mut FileData> {
        self.files.get_mut(&self.active_file)
    }

    pub fn get_active_file(&self) -> Option<&FileData> {
        self.files.get(&self.active_file)
    }

    pub fn save_active_file(&self) {
        let active_file = &self.get_active_file();

        if active_file.is_some() {
            let FileData { content, .. } = active_file.unwrap();
            fs::write(self.active_file.to_string(), content).expect("Unable to write file");
        }
    }

    pub fn close_file(&mut self, file_path: &String) {
        if self.files.contains_key(file_path) {
            self.files.remove(file_path);
        }
    }
}
