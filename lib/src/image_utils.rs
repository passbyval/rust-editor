#[macro_export]
macro_rules! load_image {
    ($path:expr $(,)?) => {
        Image::new($crate::egui::include_image!($path))
            .texture_options(TextureOptions::NEAREST)
            .maintain_aspect_ratio(true)
    };
}

pub(crate) use load_image;
