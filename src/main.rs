use eframe::{
    egui::{
        style::{Visuals, Widgets},
        Context, CursorIcon, FontData, FontDefinitions, FontFamily, ViewportBuilder,
    },
    run_native, App as EFrameApp, CreationContext, Error, Frame, NativeOptions,
};

mod themes;
#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use lib::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    use eframe::egui::*;
    pub use lib::State;

    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[derive(Default)]
pub struct App {
    state: State,
}

fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    let fira_code = include_bytes!("fonts/FiraCode-Regular.ttf");
    let fira_code_bold = include_bytes!("fonts/FiraCode-Bold.ttf");
    let roboto = include_bytes!("fonts/Roboto-Regular.ttf");

    fonts
        .font_data
        .insert("fira_code".to_owned(), FontData::from_static(fira_code));

    fonts.font_data.insert(
        "fira_code_bold".to_owned(),
        FontData::from_static(fira_code_bold),
    );

    fonts
        .font_data
        .insert("roboto".to_owned(), FontData::from_static(roboto));

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "roboto".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "fira_code".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(1, "fira_code_bold".to_owned());

    ctx.set_fonts(fonts);
}

impl EFrameApp for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        render(&mut self.state, ctx, frame);
    }
}

impl App {
    fn new(cc: &CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_zoom_factor(1.4);

        cc.egui_ctx.set_visuals(Visuals {
            widgets: Widgets {
                ..Default::default()
            },
            image_loading_spinners: true,
            interact_cursor: Some(CursorIcon::PointingHand),
            ..Default::default()
        });

        Self {
            state: State::default(),
        }
    }
}

fn main() -> Result<(), Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1200.0, 880.0])
            .with_resizable(true) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };

    run_native(
        "Rust Code Editor",
        native_options,
        Box::new(|cc| {
            #[cfg(feature = "reload")]
            {
                let ctx = cc.egui_ctx.clone();

                std::thread::spawn(move || loop {
                    hot_lib::subscribe().wait_for_reload();
                    ctx.request_repaint();
                });
            }
            Ok(Box::new(App::new(cc)))
        }),
    )
}
