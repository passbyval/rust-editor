use eframe::egui::{Context, Galley, Pos2, Rect, Vec2};
use std::sync::Arc;

pub fn get_display_size(ctx: &Context, size: Vec2) -> Vec2 {
    Vec2 {
        x: size.x / ctx.pixels_per_point(),
        y: size.y / ctx.pixels_per_point(),
    }
}

pub fn position_left_by_size(from: Rect, size: Vec2) -> Rect {
    Rect::from_center_size(
        Pos2 {
            x: from.left_center().x - size.x,
            ..from.left_center()
        },
        size,
    )
}

pub fn position_top_by_galley(from: Rect, galley: Arc<Galley>) -> Rect {
    Rect::from_center_size(
        Pos2 {
            y: from.center_top().y - galley.size().y,
            x: from.center_top().x,
        },
        galley.rect.size(),
    )
}

pub fn position_bottom_by_galley(from: Rect, galley: Arc<Galley>) -> Rect {
    Rect::from_center_size(
        Pos2 {
            y: from.center_bottom().y + galley.size().y + 15.0,
            ..from.center_bottom()
        },
        galley.rect.size(),
    )
}

#[macro_export]
macro_rules! infer_size {
    ($ui:expr, $closure:expr) => {{
        let create = || $closure;
        let (position, galley, response) = create().layout_in_ui($ui);

        (create, galley, position, response)
    }};
}

pub(crate) use infer_size;
