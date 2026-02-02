use std::sync::Arc;

// Dependencies
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
// Local modules
//mod mesh;
mod state;
use state::State;

/// Handle for a graphical application.
#[derive(Default)]
struct App {
    /// The graphical state of [`App`]
    state: Option<State>,
}

impl ApplicationHandler for App {
    /// Creates the window and event loop
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Creating new Window");
        let window_attributes = Window::default_attributes().with_title("A fantastic window!");
        let window = event_loop.create_window(window_attributes).unwrap();
        let window = Arc::new(window);
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
        info!("Window was created.");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        trace!("Received window event: {:?}", event);
        match event {
            WindowEvent::Focused(v) => {
                if v == true {
                    info!("Window {:?} was focused.", id);
                }
            },
            WindowEvent::RedrawRequested => {
                let _ = state.render();
            },
            WindowEvent::Resized(size) => {
                state.resize(size);
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => {
                trace!("Key event: code = {:?}, is_pressed = {:?}", code, key_state.is_pressed());
                match (code, key_state.is_pressed()) {
                    (KeyCode::KeyQ, true) => event_loop.exit(),
                    _ => (),
                }
            },
            WindowEvent::CloseRequested => {
                info!("Window is now closing.");
                event_loop.exit();
            },
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();
    info!("App was started.");
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
