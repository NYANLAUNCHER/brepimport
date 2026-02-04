use std::{iter, sync::Arc};

// Dependencies
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wgpu::VertexBufferLayout;
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
    /// Pipeline info specific to State
    pipeline_info: PipelineInfo,
    //vertex_layout: VertexBufferLayout<'static>,
    //vertex_buffer: Option<wgpu::Buffer>,
    //index_buffer: Option<wgpu::Buffer>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// Struct used for State::create_pipeline()
/// Makes it easy to pass State pipeline info around
pub struct PipelineInfo {
    vertex_layout: VertexBufferLayout<'static>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    /// The function name for the vertex entry point
    vertex_entry: Option<&'static str>,
    /// The function name for the fragment entry point
    fragment_entry: Option<&'static str>,
}

impl State {
    /// Associated function for creating a render pipeline for State
    pub fn create_pipeline(
        pipeline_info: PipelineInfo
    ) -> wgpu::RenderPipeline {
    }
    /// Creates a new graphics pipeline for [`super::App`]
    ///
    /// The graphics pipeline creation process consists of these discrete stages:
    ///     1. Device & Queue setup
    ///     2. Surface Configuration
    ///     3. Pipeline Creation
    ///     4. Window Attachment
    pub async fn new(
        window: Arc<Window>,
        // If this isn't specified, you must later call update_pipeline() before running render()
        vertex_layout: Option<VertexBufferLayout<'static>>,
    ) -> anyhow::Result<Self> {
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
        // Pipeline & Buffer Creation: {{{
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
                buffers: &[vertex_layout.clone()],
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
            pipeline_info,
        })
    }

    /// Use a different render pipeline
    pub fn update_pipeline(
        &mut self,
        pipeline: wgpu::RenderPipeline,
        vertex_layout: VertexBufferLayout<'static>,
        vertex_buffer: Option<wgpu::Buffer>,
        index_buffer: Option<wgpu::Buffer>,
    ) -> Result<(), anyhow::Error> {
        self.pipeline = pipeline;
        self.vertex_layout = vertex_layout;
        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
        Ok(())
    }

    /// Resize Surface to match window size.
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let width = size.width;
        let height = size.height;
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    /// Updates internal state for render().
    ///
    /// Used to update things like matrices and vectors.
    pub fn update(&mut self) {}

    /// Renders a Model object.
    //pub fn render_model<T>(&mut self, mesh: Model<T>) -> Result<(), wgpu::SurfaceError> {
    //}

    /// Renders to Surface. Uses self.vertex_buffer & self.index_buffer.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                multiview_mask: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            if let Some(vertex_buffer) = &self.vertex_buffer {
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            } else {
                warn!("State.render(): No vertex_buffer was specified in struct `State`.");
            }

            if let Some(index_buffer) = &self.index_buffer {
                let index_count = 1u32;
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..index_count, 0, 0..1);
            }
        }
        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
