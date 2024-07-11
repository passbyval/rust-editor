#![recursion_limit = "256"]

use components::{
    default_message_modal::OpenFolderCard as DefaultMessage, selectable_label::SelectableLabel,
};
use egui::{
    self, emath::RectTransform, menu, scroll_area::ScrollBarVisibility, text::Fonts, vec2, Button,
    CentralPanel, Color32, Context, CursorIcon, Frame, Label, Link, Margin, Pos2, Rect, RichText,
    ScrollArea, Sense, SidePanel, Stroke, TextEdit, TextStyle, TopBottomPanel, Ui, Vec2,
};
use file_store::FileData;
use layout::get_responsive_size;
use lazy_static::lazy_static;
use std::{fs::DirEntry, path::Path};

mod file_menu;
mod file_store;
mod file_tree;
mod file_utils;
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
    file_store: file_store::FileStore,
    theme: theme::Theme,
}

impl Default for State {
    fn default() -> Self {
        let paths = file_utils::map_paths(Path::new("./"));
        let file_list = file_store::FileStore::new();
        let theme = theme::Theme::dark();

        State {
            paths,
            file_store: file_list,
            theme,
        }
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
                file_tree::create(ui, &mut state.paths, &mut state.file_store);
            });
        });

    CentralPanel::default()
        .frame(*CENTRAL_PANE_FRAME)
        .show(ctx, |ui| {
            if state.file_store.files.is_empty() {
                let Pos2 { x, y } = ui.available_rect_before_wrap().center();

                let max_rect = Rect::from_center_size(
                    Pos2 { y: y - 80.0, x },
                    get_responsive_size(Vec2 { x, y }, ui, None),
                );

                let open_folder_card = ui.put(max_rect, DefaultMessage::new());

                if open_folder_card.clicked() {
                    file_utils::open_file(state, "/");
                }
            } else {
                ScrollArea::horizontal()
                    .id_source("tabs-scroll-container")
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let mut files = state.file_store.files.clone().into_iter().peekable();

                            while let Some((path, FileData { name, .. })) = files.next() {
                                ui.spacing_mut().item_spacing = Vec2::default();

                                let label = ui.add(
                                    SelectableLabel::new(
                                        path == state.file_store.active_file,
                                        RichText::new(&name),
                                    )
                                    .rounding(theme::Theme::ROUNDING_NONE)
                                    .padding(Vec2 {
                                        x: 26.0,
                                        ..theme::Theme::PADDING
                                    }),
                                );

                                if files.peek().is_some() {
                                    let available_space = if ui.is_sizing_pass() {
                                        Vec2::ZERO
                                    } else {
                                        ui.available_size_before_wrap()
                                    };

                                    let size = vec2(0.0, available_space.y);
                                    let (rect, response) =
                                        ui.allocate_at_least(size, Sense::hover());

                                    if ui.is_rect_visible(response.rect) {
                                        let stroke =
                                            Stroke::new(1.0, state.theme.action.disabled_bg);
                                        let painter = ui.painter();
                                        painter.vline(
                                            painter.round_to_pixel(rect.center().x),
                                            (rect.top())..=(rect.bottom()),
                                            stroke,
                                        );
                                    }
                                }

                                let to_screen = RectTransform::from_to(
                                    Rect::from_min_size(Pos2::ZERO, label.rect.size()),
                                    label.rect,
                                );

                                if label.clicked() {
                                    state.file_store.set_active_file(&path);
                                }

                                ui.add_visible_ui(ui.rect_contains_pointer(label.rect), |ui| {
                                    let size = 5.0;

                                    let position = Pos2 {
                                        x: label.rect.width() - 13.0,
                                        y: label.rect.height() / 2.0,
                                    };

                                    let max_rect = Rect {
                                        min: to_screen.transform_pos(position),
                                        max: to_screen.transform_pos(Pos2 {
                                            x: position.x,
                                            y: -size,
                                        }),
                                    };

                                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                                        Color32::TRANSPARENT;
                                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                                        state.theme.action.hover.gamma_multiply(0.3);

                                    let button = Button::new(
                                        RichText::new("x").color(state.theme.text_color.primary),
                                    )
                                    .rounding(state.theme.rounding)
                                    .stroke(Stroke::NONE);

                                    let response = ui
                                        .put(max_rect, button)
                                        .on_hover_cursor(CursorIcon::PointingHand);

                                    if response.clicked() {
                                        state.file_store.close_file(&path);
                                    }
                                    response
                                });
                            }
                        });
                    });

                if !state.file_store.active_file.is_empty() {
                    let (.., max_rect) = ui.allocate_space(ui.available_size_before_wrap());

                    ui.spacing_mut().item_spacing = Vec2::default();

                    ui.allocate_ui_at_rect(max_rect, |ui| {
                        egui::Frame::none()
                            .fill(Color32::BLACK)
                            .inner_margin(vec2(5.0, 4.0))
                            .outer_margin(vec2(0.0, 0.0))
                            .stroke(Stroke::NONE)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());

                                let mut components = state
                                    .file_store
                                    .get_active_file_components()
                                    .into_iter()
                                    .peekable();

                                ui.horizontal(|ui| {
                                    while let Some((dirent_name, file_path)) = components.next() {
                                        ui.style_mut().visuals.hyperlink_color =
                                            state.theme.text_color.primary;

                                        ui.add(Link::new(RichText::new(dirent_name)));

                                        if components.peek().is_some() {
                                            ui.add(Label::new(
                                                RichText::new(format!(
                                                    "{:^width$}",
                                                    "/",
                                                    width = 5
                                                ))
                                                .color(state.theme.text_color.primary),
                                            ));
                                        }
                                    }
                                });
                            });
                    });

                    let id = state.file_store.get_active_file_id().into();

                    match state.file_store.get_active_file_as_mut() {
                        Some(active_file) => {
                            ScrollArea::vertical().show(ui, |ui| {
                                let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                                    let mut layout_job =
                                        syntax_highlighter::highlight(string.to_string());
                                    layout_job.wrap.max_width = wrap_width;

                                    ui.fonts(|f: &Fonts| f.layout_job(layout_job))
                                };

                                if !active_file.content.is_empty() {
                                    ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                                    ui.style_mut().visuals.selection.stroke = Stroke::NONE;

                                    ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(&mut active_file.content)
                                            .id(id)
                                            .font(TextStyle::Monospace)
                                            .code_editor()
                                            .desired_rows(10)
                                            .lock_focus(true)
                                            .layouter(&mut layouter)
                                            .margin(Margin::symmetric(5.0, 5.0)),
                                    );
                                }
                            });
                        }
                        None => println!("todo"),
                    }
                }
            }
        });
}
