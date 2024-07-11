use egui::{
    Color32, CursorIcon, NumExt, Response, Rounding, Sense, Stroke, TextStyle, Ui, Vec2, Widget,
    WidgetInfo, WidgetText, WidgetType,
};

use theme::Theme;

pub struct SelectableLabel {
    selected: bool,
    text: WidgetText,
    rounding: Option<Rounding>,
    padding: Option<Vec2>,
}

impl SelectableLabel {
    pub fn new(selected: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            selected,
            text: text.into(),
            rounding: Some(Theme::ROUNDING),
            padding: Some(Theme::PADDING),
        }
    }

    pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
        self.rounding = Some(rounding.into());
        self
    }

    pub fn padding(mut self, padding: impl Into<Vec2>) -> Self {
        self.padding = Some(padding.into());
        self
    }
}

impl Widget for SelectableLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            selected,
            text,
            rounding,
            padding,
        } = self;

        let theme = Theme::dark();
        let button_padding = padding.unwrap_or(Theme::PADDING);
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let galley = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_extra + galley.size();
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                galley.text(),
            )
        });

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                .min;

            let fill: Color32 = {
                if ui.rect_contains_pointer(response.rect) {
                    theme.action.hover
                } else if selected {
                    theme.action.selected
                } else if response.has_focus() {
                    theme.action.focus
                } else {
                    Color32::BLACK
                }
            };

            ui.painter()
                .rect(rect, rounding.unwrap(), fill, Stroke::NONE);

            ui.painter()
                .galley(text_pos, galley, theme.secondary.contrast_text);
        }

        if !selected {
            return response.on_hover_cursor(CursorIcon::PointingHand);
        }

        response
    }
}
