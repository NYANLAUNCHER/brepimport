mod mesh;
mod prelude;
mod state;
// STD
use std::sync::Arc;

// Dependencies
use bytemuck::{Pod, Zeroable};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use wgpu::VertexAttribute;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

// Local
use prelude::*;
use state::{PipelineInfo, ShaderInfo, State};

/// Handle for a graphical application.
#[derive(Default)]
struct App<'a> {
    /// The graphical state of [`App`]
    state: Option<State<'a>>,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MyVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl MyVertex {
    const ATTRIBUTES: [VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,// position
        1 => Float32x3,// color
    ];
}

pub trait Vertex<'a> {
    fn layout() -> wgpu::VertexBufferLayout<'a>;
}

impl<'a> Vertex<'a> for MyVertex {
    fn layout() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

// Winding: CCW
static VERTEX_DATA: &[MyVertex] = &[
    // Top Center
    MyVertex {
        position: [0.0, 0.5, 0.1],
        color: [1.0, 0.0, 0.0],
    },
    // Bottom Left
    MyVertex {
        position: [-0.5, -0.5, 0.1],
        color: [0.0, 1.0, 0.0],
    },
    // Bottom Right
    MyVertex {
        position: [0.5, -0.5, 0.1],
        color: [0.0, 0.0, 1.0],
    },
];

impl ApplicationHandler<state::ResourceEvent<'static>> for App<'_> {
    /// Creates the window and event loop
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Creating new Window");
        let window_attributes = Window::default_attributes().with_title("A fantastic window!");
        let window = event_loop.create_window(window_attributes).unwrap();
        let window = Arc::new(window);

        let info = PipelineInfo {
            vertex_layout: MyVertex::layout(),
            vertex_buffer_init: wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTEX_DATA),
                usage: wgpu::BufferUsages::VERTEX,
            },
            index_buffer_init: (0, None),
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            shader_info: ShaderInfo {
                desc: wgpu::ShaderModuleDescriptor {
                    label: Some("Shader Model"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
                },
                vertex_entry: Some("vs_main"),
                fragment_entry: Some("fs_main"),
            },
        };
        self.state = Some(pollster::block_on(State::new(window, info)).unwrap());
        info!("Window was created.");
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: state::ResourceEvent<'static>) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };
        state.handle_event(event).expect("Couldn't handle user event.");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };
        trace!("Received window event: {:?}", event);
        match event {
            WindowEvent::Focused(v) => {
                if v == true {
                    info!("Window {:?} was focused.", id);
                }
            },
            WindowEvent::RedrawRequested => match state.render() {
                Err(e) => {
                    error!("state.render() returned error: {:?}", e);
                    panic!();
                },
                _ => (),
            },
            WindowEvent::Resized(size) => {
                state.resize(size);
            },
            WindowEvent::CloseRequested => {
                info!("Window is now closing.");
                event_loop.exit();
            },
            WindowEvent::MouseInput {
                button,
                state: button_state,
                ..
            } => {
                if log_mouse_event() {
                    debug!(
                        "Mouse event: button = {:?}, is_pressed = {:?}",
                        button, button_state
                    );
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                if log_mouse_event() {
                    debug!("Mouse event: position = {:?}", position);
                }
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                if log_key_event() {
                    debug!(
                        "Key event: code = {:?}, is_pressed = {:?}",
                        code,
                        key_state.is_pressed()
                    );
                }
                match (code, key_state.is_pressed()) {
                    (KeyCode::KeyQ, true) => event_loop.exit(),
                    _ => (),
                }
            },
            _ => (),
        }
    }
}

const fn log_mouse_event() -> bool {
    false
}

const fn log_key_event() -> bool {
    true
}

fn main() -> Result<()> {
    env_logger::init();
    info!("App was started.");
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
