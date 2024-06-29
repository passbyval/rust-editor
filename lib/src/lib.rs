#![recursion_limit = "256"]

use eframe::egui::{
    self, gui_zoom, menu, CentralPanel, Context, Key, KeyboardShortcut, Modifiers, ScrollArea,
    SidePanel, TextEdit, TextStyle, TopBottomPanel, Ui,
};

use eframe::egui::text::{Fonts, LayoutJob};
use rfd::FileDialog;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
mod syntax_highlighter;

struct File {
    content: String,
    path: String,
}

struct FileList {
    active_file: String,
    files: HashMap<String, File>,
}

impl FileList {
    pub fn new() -> Self {
        Self {
            active_file: "".into(),
            files: HashMap::new(),
        }
    }

    fn insert(&mut self, path: String) {
        let buff = fs::read(&path).expect("Should have been able to read the file");
        let content = String::from_utf8_lossy(&buff).to_string();

        let file = File {
            content,
            path: path.clone(),
        };

        self.files.insert(path.to_string(), file);
    }

    fn set_active_file(&mut self, file_path: &String) {
        self.active_file = file_path.to_string();
    }

    fn get_active_content(&mut self) -> Option<&mut File> {
        self.files.get_mut(&self.active_file)
    }

    fn save_active_file(&mut self) {
        let file = &self.get_active_content();

        match file {
            Some(file) => {
                fs::write(&file.path, &file.content).expect("Unable to write file");
            }
            _ => (),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    language: String,
    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>,
    paths: Vec<DirEntry>,
    files: FileList,
}

fn map_paths(path: &Path) -> Vec<DirEntry> {
    fs::read_dir(path).unwrap().map(|p| p.unwrap()).collect()
}

impl Default for State {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let paths = map_paths(Path::new("./"));

        State {
            tx,
            rx,
            language: "rs".into(),
            paths,
            files: FileList::new(),
        }
    }
}

fn file_menu_button(ui: &mut Ui, state: &mut State) {
    let organize_shortcut = KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::O);
    let reset_shortcut = KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::R);
    let save_shortcut_ctrl = KeyboardShortcut::new(Modifiers::CTRL, Key::S);
    let save_shortcut_meta = KeyboardShortcut::new(Modifiers::MAC_CMD, Key::S);

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    if ui.input_mut(|i| i.consume_shortcut(&save_shortcut_meta)) {
        state.files.save_active_file()
    }

    if ui.input_mut(|i| i.consume_shortcut(&save_shortcut_ctrl)) {
        state.files.save_active_file()
    }

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        if ui.button("Open").clicked() {
            let file = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                .add_filter("js", &["js", "jsx", "tsx", "ts", "cjs"])
                .set_directory("/")
                .pick_file();

            let data = file.unwrap();
            let file_path = data.into_os_string().into_string().unwrap();

            state.files.set_active_file(&file_path);
            state.files.insert(file_path);
            ui.close_menu();
        }

        if ui.button("Save").clicked() {
            state.files.save_active_file()
        }
    });

    #[cfg(not(target_arch = "wasm32"))]
    ui.menu_button("View", |ui| {
        // On the web the browser controls the zoom
        {
            gui_zoom::zoom_menu_buttons(ui);
            ui.weak(format!(
                "Current zoom: {:.0}%",
                100.0 * ui.ctx().zoom_factor()
            ))
            .on_hover_text("The UI zoom level, on top of the operating system's default value");
        }
    });
}

fn file_tree(ui: &mut Ui, paths: &mut Vec<DirEntry>, mut files: &mut FileList) {
    paths.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .is_dir()
            .cmp(&a.metadata().unwrap().is_dir())
    });

    for path in &mut *paths {
        let file_name = path.file_name().into_string().unwrap();

        if path.metadata().unwrap().is_dir() {
            let path_buff: std::path::PathBuf = path.path();
            let mut new_paths = map_paths(&path_buff);

            ui.collapsing(file_name, |inner_ui| {
                file_tree(inner_ui, &mut new_paths, &mut files)
            });
        } else {
            if ui.selectable_label(false, file_name).double_clicked() {
                print!("Double clicked!!");
                let file_path: String = path.path().into_os_string().into_string().unwrap();

                files.set_active_file(&file_path);
                files.insert(file_path);
            }
        }
    }
}

#[no_mangle]
pub fn render(state: &mut State, ctx: &Context, _frame: &mut eframe::Frame) {
    // if let Ok(code) = state.rx.try_recv() {
    //     state.code = code;
    //     state.changed = true;
    // }

    self::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            file_menu_button(ui, state);
        });
    });

    SidePanel::left("file_explorer")
        .resizable(true)
        .default_width(200.0)
        .min_width(200.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                file_tree(ui, &mut state.paths, &mut state.files);
            });
        });

    let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
        let mut layout_job: LayoutJob = syntax_highlighter::highlight(string.into());
        layout_job.wrap.max_width = wrap_width;
        ui.fonts(|f: &Fonts| f.layout_job(layout_job))
    };

    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            let file = state.files.get_active_content();

            match file {
                Some(file) => {
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut file.content)
                            .font(TextStyle::Monospace)
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .layouter(&mut layouter),
                    );
                }
                _ => (),
            }
        });
    });
}
