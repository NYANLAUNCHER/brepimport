/// Trait to implement vertex buffers
pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable {
    /// Returns the layout of the vertex attributes.
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a>;

    /// Return an immutable reference to a byte array containing the raw vertex buffer data.
    fn data<'a>(&self) -> &'a [u8];
}

/// A mesh resource handle for wgpu that guarantees vertex layout uniformity.
///
/// Created using [`crate::State::alloc_mesh()`] which sub-allocates vertex_view and index_view within the pipeline resource.
pub struct Mesh<'a, V: Vertex> {
    /// The device the mesh is being stored at
    device: &'a wgpu::Device,
    vertex_view: wgpu::BufferView,
    vertex_size: u32,
    index_view: Option<wgpu::BufferView>,
    index_size: u32,
    _marker: std::marker::PhantomData<V>,
}

/// Functions and methods for loading and manipulating raw mesh data on a wgpu device.
impl<'a, V: Vertex> Mesh<'a, V> {
    // Allocates a new mesh resource on the device.
    //pub fn new(device: &'a wgpu::Device, vertices: Option<&'a [V]>, indices: Option<&'a [u32]>) -> Self {
    //    Self {
    //        device,
    //    }
    //}
}

/// Contains a Mesh handle and a corresponding transform matrix
pub struct Model<'a, V: Vertex> {
    mesh: &'a Mesh<'a, V>,
    trans: cgmath::Matrix4<u32>,
}
