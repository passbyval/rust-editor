#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use lib::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    use eframe::egui;
    pub use lib::State;

    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

use eframe::egui::{
    self, CentralPanel, Context, DragValue, Event, ScrollArea, SidePanel, TextEdit, TextStyle, Vec2,
};

#[derive(Default)]
pub struct MyApp {
    state: State,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        render(&mut self.state, ctx, frame);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 480.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // When hot reload is enabled, repaint after every lib change
            #[cfg(feature = "reload")]
            {
                let ctx = cc.egui_ctx.clone();
                std::thread::spawn(move || loop {
                    hot_lib::subscribe().wait_for_reload();
                    ctx.request_repaint();
                });
            }
            Box::new(MyApp::default())
        }),
    );
}
