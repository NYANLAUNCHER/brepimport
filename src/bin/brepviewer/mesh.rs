// Dependencies
use wgpu::{VertexBufferLayout, util::DeviceExt};

/// Trait to implement vertex buffers
///
/// # Example Boilerplate
/// ```
/// use bytemuck::{Pod, Zeroable};
/// #[repr(C)]
/// #[derive(Copy, Clone, Debug, Pod, Zeroable)]
/// /// Contains a position and a color attribute
/// struct MyVertex<'a> {
///     /// Position attribute. Assumes W component = 1.0.
///     position: [f32; 3],
///     /// Color attribute. Assumes alpha = 1.0.
///     color: [f32; 3],
/// }
/// impl Vertex for MyVertex {
///     /// See: [`Vertex::layout()`] for implementation example
///     fn layout(&self) -> wgpu::VertexBufferLayout<'static> {/*...*/}
///
///     /// See: [`Vertex::data()`] for implementation example
///     fn data<'a>(&self) -> &'a [u8] {/*...*/}
/// }
/// ```
pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    /// Returns the layout of the vertex attributes.
    ///
    /// # Example
    /// ```
    /// fn layout() -> wgpu::VertexBufferLayout<'static> {
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

/// A mesh resource handle for wgpu.
///
/// Guarantees vertex layout uniformity. Doesn't support buffer suballocation.
pub struct Mesh<V: Vertex> {
    /// The device the mesh is being stored at
    device: wgpu::Device,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: Option<wgpu::Buffer>,
    index_count: u32,
    _marker: std::marker::PhantomData<V>,
}

/// Functions and methods for loading and manipulating raw mesh data on a wgpu device.
impl<V: Vertex> Mesh<V> {
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
    pub fn from(device: wgpu::Device, vertices: &[V], indices: &[u16]) -> Self {
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
}

/// Contains a Mesh handle and a corresponding transform matrix
pub struct Model<V: Vertex> {
    mesh: Mesh<V>,
    trans: cgmath::Matrix4<u32>,
}
