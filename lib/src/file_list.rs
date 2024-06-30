use std::collections::HashMap;
use std::fs::{self};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    pub content: String,
    pub path: String,
    pub name: String,
}

pub struct FileList {
    pub active_file_path: String,
    pub active_file_name: String,
    pub files: HashMap<String, File>,
}

impl FileList {
    pub fn new() -> Self {
        Self {
            active_file_path: "".into(),
            active_file_name: "".into(),
            files: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file_path: &String, set_active: bool) -> () {
        let path_buf = PathBuf::from(file_path).to_path_buf();
        let file_name = FileList::get_file_name(&path_buf);
        let file_path = FileList::get_file_path(&path_buf);

        let file = File {
            content: match self.files.get(&file_path) {
                Some(file) => file.content.to_string(),
                None => {
                    let buff = fs::read(file_path.to_string())
                        .expect("Should have been able to read the file");
                    String::from_utf8_lossy(&buff).to_string()
                }
            },
            path: file_path.to_string(),
            name: file_name,
        };

        if set_active {
            self.set_active_file(&file_path);
        }

        self.files.insert(file_path.clone(), file);
    }

    pub fn get_file_path(path_buf: &PathBuf) -> String {
        path_buf.clone().into_os_string().into_string().unwrap()
    }

    pub fn get_file_name(path_buf: &PathBuf) -> String {
        path_buf
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap()
    }

    pub fn set_active_file(&mut self, file_path: &String) {
        self.active_file_path = file_path.to_string();
        self.active_file_name = FileList::get_file_name(&PathBuf::from(file_path).to_path_buf())
    }

    pub fn get_active_content(&mut self) -> Option<&mut File> {
        self.files.get_mut(&self.active_file_path)
    }

    pub fn save_active_file(&mut self) {
        let file = &self.get_active_content();

        match file {
            Some(file) => {
                fs::write(&file.path, &file.content).expect("Unable to write file");
            }
            _ => (),
        }
    }
}
