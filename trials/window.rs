use egui::util::cache;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use eframe::egui;
use std::sync::Arc;

use glyphon::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer,
};

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use wgpu::{
    include_wgsl, Backends, BlendState, Color as WGpuColor, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Face, Features,
    FragmentState, FrontFace, Instance, InstanceDescriptor, Limits, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PresentMode,
    PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule, StoreOp,
    Surface, SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages,
    TextureViewDescriptor, VertexState,
};

pub struct State<'window> {
    surface: Surface<'window>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    pub window: Arc<Window>,
    render_pipeline: RenderPipeline,
    scale_factor: f64,
    font_system: FontSystem,
    text_renderer: TextRenderer,
    text_atlas: TextAtlas,
    cache: SwashCache,
}

impl State<'_> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let swapchain_format: TextureFormat = TextureFormat::Bgra8UnormSrgb;

        let surface = instance
            .create_surface(window.clone())
            .expect("Create surface");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        Limits::downlevel_webgl2_defaults()
                    } else {
                        Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        surface.configure(&device, &config);

        let (font_system, text_renderer, text_atlas, cache) =
            Self::create_text_renderer(swapchain_format, &device, &queue);

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let shader: ShaderModule = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[],           // 2.
            },
            fragment: Some(FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: FrontFace::Ccw, // 2.
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            scale_factor,
            text_renderer,
            text_atlas,
            font_system,
            cache,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn create_text(&mut self) -> Buffer {
        let mut buffer: Buffer = Buffer::new(&mut self.font_system, Metrics::new(30.0, 42.0));

        let physical_width = (self.size.width as f64 * self.scale_factor) as f32;
        let physical_height = (self.size.height as f64 * self.scale_factor) as f32;

        buffer.set_size(&mut self.font_system, physical_width, physical_height);
        buffer.set_text(&mut self.font_system, "Hello world! 👋\nThis is rendered with 🦅 glyphon 🦁\nThe text below should be partially clipped.\na b c d e f g h i j k l m n o p q r s t u v w x y z", Attrs::new().family(Family::SansSerif), Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system);

        buffer
    }

    fn create_text_renderer(
        swapchain_format: TextureFormat,
        device: &Device,
        queue: &Queue,
    ) -> (FontSystem, TextRenderer, TextAtlas, SwashCache) {
        let font_system = FontSystem::new();
        let cache = SwashCache::new();

        let mut atlas = TextAtlas::new(device, queue, swapchain_format);
        let text_renderer =
            TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);

        (font_system, text_renderer, atlas, cache)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let texture = self.surface.get_current_texture()?;
        let buffer = self.create_text();

        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.text_atlas,
                Resolution {
                    width: self.config.width,
                    height: self.config.height,
                },
                [TextArea {
                    buffer: &buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: 600,
                        bottom: 160,
                    },
                    default_color: Color::rgb(255, 255, 255),
                }],
                &mut self.cache,
            )
            .unwrap();

        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(WGpuColor::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.text_renderer
            .render(&self.text_atlas, &mut render_pass)
            .unwrap();

        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
        self.text_atlas.trim();

        Ok(())
    }
}
