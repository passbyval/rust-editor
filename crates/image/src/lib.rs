#[macro_export]
macro_rules! load_image {
    ($path:expr $(,)?) => {
        egui::Image::new(egui::include_image!($path)).texture_options(egui::TextureOptions::NEAREST)
    };
}
