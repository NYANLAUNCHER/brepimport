use std::path::Path;

use wgpu::{util::DeviceExt, wgc::id::markers::Device};

//#[repr(C)]
//#[derive(Copy, Clone, Debug)]
//struct Vertex {
//    data: &'static [u8]
//}
//unsafe impl bytemuck::Pod for Vertex {}
//unsafe impl bytemuck::Zeroable for Vertex {}

pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    /// Return the layout of the vertex attributes
    ///
    /// # Example
    /// ```
    /// pub fn layout() -> wgpu::VertexBufferLayout<'static> {
    ///     wgpu::VertexBufferLayout {
    ///         array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
    ///         step_mode: wgpu::VertexStepMode::Vertex,
    ///         attributes: &[
    ///             // Position
    ///             wgpu::VertexAttribute {
    ///                 offset: 0,
    ///                 shader_location: 0,
    ///                 format: wgpu::VertexFormat::Float32x3,
    ///             },
    ///             // Color
    ///             wgpu::VertexAttribute {
    ///                 // offset from the start of the previous attribute
    ///                 offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
    ///                 shader_location: 1,
    ///                 format: wgpu::VertexFormat::Float32x2,
    ///             },
    ///         ],
    ///     }
    /// }
    /// ```
    fn layout(&self) -> wgpu::VertexBufferLayout<'static>;


    /// Return a reference to a byte array containing the raw vertex buffer data.
    fn data<'a>(&self) -> &'a [u8];
}

/// A mesh resource handle for wgpu. Guarantees vertex layout uniformity.
pub struct Mesh<T: Vertex> {
    /// The device the mesh is being stored at
    device: wgpu::Device,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: Option<wgpu::Buffer>,
    index_count: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Vertex> Mesh<T> {
    /// Allocates a new mesh resource on the device.
    pub fn new(device: wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh: Vertex Buffer"),
            contents: &[0],
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh: Index Buffer"),
            contents: &[0],
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            device,
            vertex_buffer,
            vertex_count: 0,
            index_buffer: Some(index_buffer),
            index_count: 0,
            _marker: std::marker::PhantomData,
        }
    }

    /// Allocates a new mesh resource on the device from a slice of vertices.
    pub fn from(device: wgpu::Device, vertices: &[T], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh: Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh: Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        Self {
            device,
            vertex_buffer,
            vertex_count: vertices.len() as u32,
            index_buffer: Some(index_buffer),
            index_count: indices.len() as u32,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a new Mesh from a file
    pub fn load(device: wgpu::Device, file_path: &Path) -> Self {
        todo!("Load file @ file_path then parse it into mesh.");
        let mesh = {
            Self::new(device)
        };
        mesh
    }
}
