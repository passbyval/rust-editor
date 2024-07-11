use egui::{Ui, Vec2};

pub fn get_responsive_size(size: Vec2, ui: &mut Ui, min_size: Option<Vec2>) -> Vec2 {
    let x = {
        let responsive_x = size.x / ui.ctx().pixels_per_point();
        let max = ui.available_size_before_wrap().x.floor().max(1.0);

        let min_x = match min_size {
            Some(size) => size.x.clamp(1.0, max),
            None => 1.0,
        };

        responsive_x.clamp(min_x, max)
    };

    let y = {
        let responsive_y = size.y / ui.ctx().pixels_per_point();
        let max = ui.available_size_before_wrap().y.floor().max(1.0);

        let min_y = match min_size {
            Some(size) => size.y.clamp(1.0, max),
            None => 1.0,
        };

        responsive_y.clamp(min_y, max)
    };

    Vec2 { x, y }
}

#[macro_export]
macro_rules! infer_size {
    ($ui:expr, $closure:expr) => {{
        let create = || $closure;
        let (position, galley, response) = create().layout_in_ui($ui);

        (create, galley, position, response)
    }};
}
