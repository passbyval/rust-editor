#![recursion_limit = "256"]

use eframe::egui::{
    self, gui_zoom, menu, CentralPanel, Context, Key, KeyboardShortcut, Modifiers, ScrollArea,
    SidePanel, TextEdit, TextStyle, TopBottomPanel, Ui,
};

use eframe::egui::text::Fonts;
use egui::emath::RectTransform;
use egui::{Color32, Pos2, Rect, RichText, Stroke};
use rfd::FileDialog;
use std::fs::{self, DirEntry};
use std::path::Path;
mod file_list;
mod syntax_highlighter;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    paths: Vec<DirEntry>,
    file_list: file_list::FileList,
}

impl Default for State {
    fn default() -> Self {
        // let (tx, rx) = mpsc::channel();
        let paths = map_paths(Path::new("./"));
        let file_list = file_list::FileList::new();

        // let (mut tx, rx) = futures_channel::mpsc::channel(1024);
        // let mut debounced = debounced(rx, Duration::from_secs(1));

        // thread::spawn(move || {
        //     let layouter = |ui: &Ui, string: &str, wrap_width: f32| {
        //         let mut layout_job: LayoutJob = syntax_highlighter::highlight(string.into());
        //         layout_job.wrap.max_width = wrap_width;
        //         ui.fonts(|f: &Fonts| f.layout_job(layout_job))
        //     };

        //     tx.send(Box::new(layouter) as Box<dyn Send + FnMut(&Ui, &str, f32) -> Arc<Galley>>)
        //         .unwrap();
        // });

        State { paths, file_list }
    }
}

fn map_paths(path: &Path) -> Vec<DirEntry> {
    fs::read_dir(path).unwrap().map(|p| p.unwrap()).collect()
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
        state.file_list.save_active_file()
    }

    if ui.input_mut(|i| i.consume_shortcut(&save_shortcut_ctrl)) {
        state.file_list.save_active_file()
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

            if file.is_some() {
                let path_buf = file.unwrap();
                let file_path = file_list::FileList::get_file_path(&path_buf);

                state.file_list.insert(&file_path, true);
            }

            ui.close_menu();
        }

        if ui.button("Save").clicked() {
            state.file_list.save_active_file()
        }
    });

    #[cfg(not(target_arch = "wasm32"))]
    ui.menu_button("View", |ui| {
        gui_zoom::zoom_menu_buttons(ui);
        ui.weak(format!(
            "Current zoom: {:.0}%",
            100.0 * ui.ctx().zoom_factor()
        ))
        .on_hover_text("The UI zoom level, on top of the operating system's default value");
    });
}

fn file_tree(ui: &mut Ui, paths: &mut Vec<DirEntry>, mut files: &mut file_list::FileList) {
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
            if ui.selectable_label(false, file_name).clicked() {
                let path_buf = path.path();
                let file_path = file_list::FileList::get_file_path(&path_buf);

                files.insert(&file_path, true);
            }
        }
    }
}

#[no_mangle]
pub fn render(state: &mut State, ctx: &Context, _frame: &mut eframe::Frame) {
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
                file_tree(ui, &mut state.paths, &mut state.file_list);
            });
        });

    CentralPanel::default().show(ctx, |ui| {
        if !state.file_list.active_file_path.is_empty() {
            ui.horizontal(|ui| {
                for (current_path, file) in &state.file_list.file_meta_data.clone() {
                    let label: egui::Response = ui.selectable_label(
                        current_path == &state.file_list.active_file_path,
                        RichText::new(format!("{:^width$}", &file.name, width = 18)),
                    );

                    let to_screen = RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, label.rect.size()),
                        label.rect,
                    );

                    if label.clicked() {
                        state.file_list.insert(current_path, true);
                    }

                    if ui.rect_contains_pointer(label.rect) {
                        let close_button = ui.put(
                            Rect {
                                min: to_screen.transform_pos(Pos2 {
                                    x: label.rect.width() - 9.0,
                                    y: 0.0,
                                }),
                                max: to_screen.transform_pos(Pos2 { x: 20.0, y: 0.0 }),
                            },
                            egui::Button::new("x")
                                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                                .fill(Color32::TRANSPARENT),
                        );

                        if close_button.clicked() {
                            state.file_list.close_file(current_path);
                        }
                    };
                }
            });
        }

        ScrollArea::vertical().show(ui, |ui| {
            let (file_path, content) = state.file_list.get_active_content();

            let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                let mut layout_job = syntax_highlighter::highlight(string.to_string());
                layout_job.wrap.max_width = wrap_width;

                ui.fonts(|f: &Fonts| f.layout_job(layout_job))
            };

            match content {
                Some(text_content) => {
                    let text_edit = ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(text_content)
                            .id(file_path.into())
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
