#[macro_export]
macro_rules! load_image {
    ($path:expr $(,)?) => {
        $crate::egui::Image::new($crate::egui::include_image!($path))
            .texture_options($crate::egui::TextureOptions::NEAREST)
    };
}

pub(crate) use load_image;
