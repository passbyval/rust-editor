#![recursion_limit = "256"]

use components::open_folder_card::OpenFolderCard;
use egui::{
    self, emath::RectTransform, menu, text::Fonts, CentralPanel, Color32, Context, Frame, Label,
    Margin, Pos2, Rect, RichText, Rounding, ScrollArea, SidePanel, Stroke, TextEdit, TextStyle,
    TopBottomPanel, Ui, Vec2,
};

use file_list::FileMetaData;
use lazy_static::lazy_static;
use std::{fs::DirEntry, path::Path};

mod components;
mod file_list;
mod file_menu;
mod file_tree;
mod file_utils;
mod image_utils;
mod layout_utils;
mod syntax_highlighter;

lazy_static! {
    static ref CENTRAL_PANE_FRAME: Frame = Frame {
        fill: egui::Visuals::dark().panel_fill,
        ..Frame::default()
    };
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    paths: Vec<DirEntry>,
    file_list: file_list::FileList,
}

impl Default for State {
    fn default() -> Self {
        let paths = file_utils::map_paths(Path::new("./"));
        let file_list = file_list::FileList::new();

        State { paths, file_list }
    }
}

#[no_mangle]
pub fn render(state: &mut State, ctx: &Context, _frame: &mut eframe::Frame) {
    self::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::bar(ui, |ui| {
            file_menu::create(ui, state);
        });
    });

    SidePanel::left("file_explorer")
        .resizable(true)
        .default_width(200.0)
        .min_width(200.0)
        .show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                file_tree::create(ui, &mut state.paths, &mut state.file_list);
            });
        });

    CentralPanel::default()
        .frame(*CENTRAL_PANE_FRAME)
        .show(ctx, |ui| {
            if state.file_list.active_file_path.is_empty() {
                let Pos2 { x, y } = ui.available_rect_before_wrap().center();

                let max_rect = Rect::from_center_size(
                    Pos2 { y: y - 80.0, x },
                    layout_utils::get_responsive_size(Vec2 { x, y }, ui, None),
                );

                let open_folder_card = ui.put(max_rect, OpenFolderCard::new());

                if open_folder_card.clicked() {
                    file_utils::open_file(state, "/");
                }
            } else {
                ui.horizontal(|ui| {
                    for (_, FileMetaData { path, name }) in state.file_list.file_meta_data.clone() {
                        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                        ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::NONE;
                        ui.style_mut().visuals.widgets.inactive.rounding = Rounding::ZERO;

                        let label: egui::Response = ui.selectable_label(
                            path == state.file_list.active_file_path,
                            RichText::new(format!("{:^width$}", &name, width = 18)),
                        );

                        let to_screen = RectTransform::from_to(
                            Rect::from_min_size(Pos2::ZERO, label.rect.size()),
                            label.rect,
                        );

                        if label.clicked() {
                            state.file_list.set_active_file(&path);
                        }

                        if ui.rect_contains_pointer(label.rect) {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                                Color32::TRANSPARENT;

                            let close_button = ui.put(
                                Rect {
                                    min: to_screen.transform_pos(Pos2 {
                                        x: label.rect.width() - 9.0,
                                        y: 4.0,
                                    }),
                                    max: to_screen.transform_pos(Pos2 { x: 20.0, y: 0.0 }),
                                },
                                egui::Button::new("x")
                                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                                    .small(),
                            );

                            if close_button.hovered() {
                                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::GRAY;
                            }

                            if close_button.clicked() {
                                state.file_list.close_file(&path);
                            }
                        };
                    }
                });

                if state.file_list.get_active_content().1.is_some() {
                    ScrollArea::vertical().show(ui, |ui| {
                        let (file_path, content) = state.file_list.get_active_content();

                        let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                            let mut layout_job = syntax_highlighter::highlight(string.to_string());
                            layout_job.wrap.max_width = wrap_width;

                            ui.fonts(|f: &Fonts| f.layout_job(layout_job))
                        };

                        match content {
                            Some(text_content) => {
                                ui.add_sized(
                                    ui.available_size(),
                                    TextEdit::multiline(text_content)
                                        .id(file_path.into())
                                        .font(TextStyle::Monospace)
                                        .code_editor()
                                        .desired_rows(10)
                                        .lock_focus(true)
                                        .layouter(&mut layouter)
                                        .margin(Margin::symmetric(5.0, 5.0)),
                                );
                            }
                            _ => println!("todo"),
                        }
                    });
                }
            }
        });
}
