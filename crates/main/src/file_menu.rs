use eframe::egui::{gui_zoom, Key, KeyboardShortcut, Modifiers, Ui};

use crate::{file_utils, State};

pub fn create(ui: &mut Ui, state: &mut State) {
    let organize_shortcut = KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::O);
    let reset_shortcut = KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::R);
    let save_shortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    if ui.input_mut(|i| i.consume_shortcut(&save_shortcut)) {
        state.file_store.save_active_file()
    }

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

        if ui.button("Open File").clicked() {
            file_utils::open_file(state, "/");
            ui.close_menu();
        }

        if ui.button("Open Folder").clicked() {
            file_utils::open_file(state, "/");
            ui.close_menu();
        }

        if ui.button("Save").clicked() {
            state.file_store.save_active_file()
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
