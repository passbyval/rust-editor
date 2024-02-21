use egui::util::cache;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use eframe::egui;
use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

use wgpu::SurfaceError;

mod window;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    pollster::block_on(run());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use log::{Level};

            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let (width, height) = (WINDOW_WIDTH, WINDOW_HEIGHT);

    let window = Arc::new(
        WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(width as f64, height as f64))
            .with_title("glyphon hello world")
            .build(&event_loop)
            .unwrap(),
    );

    let mut state = window::State::new(window).await;

    #[cfg(target_arch = "wasm32")]
    {
        use winit::{dpi::PhysicalSize, platform::web::WindowExtWebSys};

        window.request_inner_size(PhysicalSize::new(450, 400));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("root")?;
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let _ = event_loop.run(
        move |event, control_flow: &winit::event_loop::EventLoopWindowTarget<()>| match event {
            Event::WindowEvent { event, window_id } if window_id == state.window.id() => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        } => {
                            control_flow.exit();
                        }
                        WindowEvent::Resized(physical_size) => state.resize(physical_size),
                        WindowEvent::RedrawRequested => {
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(SurfaceError::Lost) => state.resize(state.size),
                                // The system is out of memory, we should probably quit
                                Err(SurfaceError::OutOfMemory) => {
                                    control_flow.exit();
                                }
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        },
    );
}
