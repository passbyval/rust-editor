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
pub struct MyApp {
    state: State,
}

fn setup_custom_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    let fira_code = include_bytes!("fonts/FiraCode-Regular.ttf");

    fonts.font_data.insert(
        "fira_code".to_owned(),
        eframe::egui::FontData::from_static(fira_code),
    );

    fonts
        .families
        .entry(eframe::egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "fira_code".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        render(&mut self.state, ctx, frame);
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        cc.egui_ctx.set_zoom_factor(1.4);

        Self {
            state: State::default(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 880.0])
            .with_resizable(true) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            #[cfg(feature = "reload")]
            {
                let ctx = cc.egui_ctx.clone();

                std::thread::spawn(move || loop {
                    hot_lib::subscribe().wait_for_reload();
                    ctx.request_repaint();
                });
            }
            Box::new(MyApp::new(cc))
        }),
    )
}
