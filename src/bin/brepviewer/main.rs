use std::time::SystemTime;
// Dependencies
use winit::{
    application::ApplicationHandler,
    window::Window,
    event_loop::EventLoop,
};
// Local modules
mod mesh;
mod state;
use state::State;

/// Handle for a graphical application.
struct App {
    /// Event loop to capture system events
    pub event_loop: EventLoop<State>,

    /// Platform dependent window handle
    pub window: Option<Window>,

    /// The state of the graphics pipeline for [`App`]
    state: Option<State>,
}

impl App {
    /// Creates a handle for a graphical app. The App isn't ran until [`App::run()`] is called.
    pub fn new() -> Self {
        let event_loop = 
        Self {
            event_loop,
            window: None,
            state: None,
        }
    }

    /// Handles window creation and state instantiation
    pub fn run(&self) -> anyhow::Result<Self> {
        let window = {()};
        let state = State::new();
        Ok(Self {
            event_loop: self.event_loop,
            window,
            state,
        })
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

fn main() {
    let brepviewer = App::new();
    brepviewer.run();
}
