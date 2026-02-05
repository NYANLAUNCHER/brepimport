use std::{iter, sync::Arc};

// Dependencies
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use winit::{dpi::PhysicalSize, window::Window};
// Local modules
//use super::mesh::Mesh;

/// Represents the graphical state of [`super::App`]
pub struct State<'a> {
    /// Platform dependent window handle
    pub window: Arc<Window>,
    /// Represents the physical graphics device or GPU.
    pub device: wgpu::Device,
    /// The GPU's work queue.
    queue: wgpu::Queue,
    /// Represents a surface on which to render graphics, see: [`wgpu::Surface`].
    surface: wgpu::Surface<'static>,
    /// Configuration for [`State::surface`].
    surface_config: wgpu::SurfaceConfiguration,
    /// The actual render pipeline, which outlines the shader and resource layouts.
    pipeline: wgpu::RenderPipeline,
    /// Pipeline info specific to State
    pipeline_info: PipelineInfo<'a>,
}

/// Info struct to create a render pipeline using [`State::create_pipeline()`].
#[derive(Clone)]
pub struct PipelineInfo<'a> {
    pub vertex_layout: wgpu::VertexBufferLayout<'a>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub shader_info: ShaderInfo<'a>,
    //pub render_info: PipelineRenderInfo,
}

///// Info struct used to render the pipeline in [`State::render()`]
//#[derive(Clone)]
//pub struct PipelineRenderInfo {
//}

/// Info struct used to create shader modules in [`State::create_pipeline()`].
#[derive(Clone)]
pub struct ShaderInfo<'a> {
    pub desc: wgpu::ShaderModuleDescriptor<'a>,
    /// The function name for the vertex entry point
    pub vertex_entry: Option<&'a str>,
    /// The function name for the fragment entry point
    pub fragment_entry: Option<&'a str>,
}

impl<'a> State<'a> {
    /// Associated function for creating a render pipeline in [`State`].
    pub fn create_pipeline(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        info: PipelineInfo,
    ) -> wgpu::RenderPipeline {//{{{
        let shader_module = device.create_shader_module(info.shader_info.desc);
        let vertex_entry = info.shader_info.vertex_entry;
        let fragment_entry = info.shader_info.fragment_entry;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        // Create the wgpu::RenderPipeline
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            // Vertex shader stage
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: vertex_entry,
                buffers: &[info.vertex_layout.clone()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            // Fragment shader stage
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: fragment_entry,
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
        })
    }
    //}}}

    /// Creates a new graphics pipeline for [`super::App`]
    ///
    /// # Overview
    /// The graphics pipeline creation process consists of these discrete stages:
    ///     1. Device & Queue setup
    ///     2. Surface Configuration
    ///     3. Pipeline Creation
    ///     4. Window Attachment
    pub async fn new(
        window: Arc<Window>,
        pipeline_info: PipelineInfo<'a>,
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
        let pipeline = Self::create_pipeline(&device, &surface_config, pipeline_info.clone());
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
    //pub fn update_pipeline(
    //    &mut self,
    //    pipeline: wgpu::RenderPipeline,
    //    pipeline_info: PipelineInfo,
    //) -> Result<(), anyhow::Error> {
    //    self.pipeline = pipeline;
    //    self.pipeline_info = pipeline_info;
    //    Ok(())
    //}

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

            let vertex_buffer = &self.pipeline_info.vertex_buffer;
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

            let index_buffer = &self.pipeline_info.index_buffer;
            if let Some(idx_buf) = index_buffer {
                let index_count = 1u32;
                render_pass.set_index_buffer(idx_buf.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..index_count, 0, 0..1);
            }
        }
        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
