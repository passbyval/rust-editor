use egui::{
    Button, Color32, CursorIcon, Frame, Label, Response, RichText, Sense, Stroke, Style,
    TextWrapMode, Ui, Vec2, Widget,
};
use image::load_image;
use layout::get_responsive_size;
use lazy_static::lazy_static;

pub struct OpenFolderCard {}

lazy_static! {
    static ref ICON_SIZE: Vec2 = Vec2 { x: 48.0, y: 48.0 };
}

impl OpenFolderCard {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_icon_size(self, ui: &mut Ui) -> Vec2 {
        let image_ratio = 24.0;
        let multiplier = 16.0;
        let true_size = image_ratio * multiplier;

        get_responsive_size(
            Vec2 {
                x: true_size,
                y: true_size,
            },
            ui,
            Some(*ICON_SIZE),
        )
    }
}

impl Widget for OpenFolderCard {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut group = Frame::group(&Style::default())
            .fill(Color32::from_rgba_unmultiplied(35, 35, 35, 35))
            .stroke(Stroke::NONE)
            .inner_margin(30.0)
            .rounding(7.0)
            .begin(ui);

        {
            group.content_ui.add_sized(
                self.get_icon_size(ui),
                load_image!("./images/folder_open.svg").max_size(Vec2::new(70.0, 70.0)),
            );

            group.content_ui.add_space(6.0);

            group
                .content_ui
                .style_mut()
                .visuals
                .widgets
                .active
                .weak_bg_fill = Color32::from_rgb(66, 165, 245);

            group
                .content_ui
                .style_mut()
                .visuals
                .widgets
                .inactive
                .weak_bg_fill = Color32::from_rgb(144, 202, 249);

            let button = group.content_ui.add_sized(
                [80.0, 30.0],
                Button::new(RichText::new("Open").color(Color32::BLACK))
                    .stroke(Stroke::NONE)
                    .rounding(4.0),
            );

            // Hack to get hovered styles working.
            if group.content_ui.rect_contains_pointer(button.rect) {
                button.request_focus();
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand)
            } else {
                button.surrender_focus();
                ui.ctx().set_cursor_icon(CursorIcon::default())
            }

            group.content_ui.add_space(15.0);
            group.content_ui.reset_style();

            group.content_ui.add(
                Label::new(
                    RichText::new("Open a file or folder to get started.").color(Color32::WHITE),
                )
                .wrap_mode(TextWrapMode::Extend),
            );

            group.content_ui.add_space(5.0);
        }

        group.paint(ui);
        group.allocate_space(ui).interact(Sense::click())
    }
}
