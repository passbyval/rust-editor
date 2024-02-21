use eframe::egui::{
    self, gui_zoom, menu, CentralPanel, Key, KeyboardShortcut, Modifiers, ScrollArea, SidePanel,
    TextEdit, TextStyle, TopBottomPanel, Ui,
};

use rfd::FileDialog;
use std::fs;
use std::sync::mpsc::{Receiver, Sender};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    language: String,
    code: String,
    picked_path: Option<String>,
    tx: Sender<String>,
    rx: Receiver<String>,
}

impl Default for State {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        State {
            tx,
            rx,
            language: "rs".into(),
            picked_path: Some(String::from("")),
            code: "".into(),
        }
    }
}

fn file_menu_button(ui: &mut Ui, state: &mut State) {
    let organize_shortcut = KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::O);
    let reset_shortcut = KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
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
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");
            state.code = contents;
        }

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
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

#[no_mangle]
pub fn render(state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if let Ok(code) = state.rx.try_recv() {
        state.code = code;
    }

    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            file_menu_button(ui, state);
        });
    });

    SidePanel::left("file_explorer").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Scroll speed")
                .on_hover_text("How fast to pan with the mouse wheel");
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut state.code)
                    .font(TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true),
            );
        });
    });
}
