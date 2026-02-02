use std::sync::Arc;

// Dependencies
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use winit::{dpi::PhysicalSize, window::Window};
// Local modules
//use super::mesh::Mesh;

/// Represents the graphical state of [`super::App`]
pub struct State {
    /// Platform dependent window handle
    pub window: Arc<Window>,
    /// Represents the physical graphics device or GPU.
    device: wgpu::Device,
    /// The GPU's work queue.
    queue: wgpu::Queue,
    /// Represents a surface on which to render graphics, see: [`wgpu::Surface`].
    surface: wgpu::Surface<'static>,
    /// Configuration for [`State::surface`].
    surface_config: wgpu::SurfaceConfiguration,
    /// The actual render pipeline, which outlines the shader and resource layouts.
    pipeline: wgpu::RenderPipeline,
}

impl State {
    /// Creates a new graphics pipeline for [`super::App`]
    ///
    /// The graphics pipeline creation process consists of these discrete stages:
    ///     1. Device & Queue setup
    ///     2. Surface Configuration
    ///     3. Pipeline Creation
    ///     4. Window Attachment
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        // API & Device Setup: {{{
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = match instance.create_surface(window.clone()) {
            Ok(val) => val,
            Err(e) => {
                panic!("Binding `surface` returned error: {:?}", e);
            }
        };

        // Adapter to filter device based on capabilities
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("DeviceDescriptor"),
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();
        //}}}
        // Surface Creation: {{{
        let window_size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };
        //}}}
        // Pipeline Creation: {{{
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            // Vertex shader stage
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // Fragment shader stage
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // Primitive shader stage
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0u64, // bitwise not; 000...0 -> 111...1
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });
        //}}}
        Ok(Self {
            window,
            device,
            queue,
            surface,
            surface_config,
            pipeline,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let width = size.width;
        let height = size.height;
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        Ok(())
    }
}
