use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileMetaData {
    pub path: String,
    pub name: String,
}

pub struct FileList {
    pub active_file_path: String,
    pub active_file_name: String,
    pub file_meta_data: HashMap<String, FileMetaData>,
    pub file_content: HashMap<String, String>,
}

impl Default for FileList {
    fn default() -> Self {
        FileList {
            active_file_path: "".into(),
            active_file_name: "".into(),
            file_meta_data: HashMap::new(),
            file_content: HashMap::new(),
        }
    }
}

impl FileList {
    pub fn new() -> Self {
        Self {
            ..FileList::default()
        }
    }

    pub fn insert(&mut self, file_path: &String, set_active: bool) -> () {
        let path_buf = PathBuf::from(file_path).to_path_buf();
        let file_name = FileList::get_file_name(&path_buf);
        let file_path = FileList::get_file_path(&path_buf);

        let content = match self.file_content.get(&file_path) {
            Some(file_content) => file_content.to_string(),
            None => {
                let buff = fs::read(&file_path).expect("Should have been able to read the file");
                String::from_utf8_lossy(&buff).to_string()
            }
        };

        let file = FileMetaData {
            path: file_path.to_string(),
            name: file_name,
        };

        if set_active {
            self.set_active_file(&file_path);
        }

        self.file_meta_data.insert(file_path.to_string(), file);
        self.file_content.insert(file_path, content);
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

    pub fn get_active_content(&mut self) -> (String, Option<&mut String>) {
        (
            self.active_file_path.to_string(),
            self.file_content.get_mut(&self.active_file_path),
        )
    }

    pub fn save_active_file(&mut self) {
        let (file_path, file_content) = self.get_active_content();

        match file_content {
            Some(content) => {
                fs::write(file_path, content).expect("Unable to write file");
            }
            None => println!("todo"),
        }
    }

    pub fn close_file(&mut self, file_path: &String) {
        self.file_meta_data.remove(file_path);
        self.file_content.remove(file_path);
        self.active_file_name = FileList::default().active_file_name;
        self.active_file_path = FileList::default().active_file_path;

        match self.file_meta_data.clone().keys().nth(0) {
            Some(active_file) => self.set_active_file(active_file),
            None => println!("todo"),
        }
    }
}
