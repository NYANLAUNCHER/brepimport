use std::sync::Arc;

// Dependencies
use log::{error, info, warn};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};
// Local modules
//mod mesh;
mod state;
use state::State;

/// Handle for a graphical application.
#[derive(Default)]
struct App {
    /// Platform dependent window handle
    pub window: Option<Window>,

    /// The state of the graphics pipeline for [`App`]
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("A fantastic window!");
        let window = event_loop.create_window(window_attributes).unwrap();
        self.state = Some(pollster::block_on(State::new(Arc::new(window))).unwrap());
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Focused(v) => {
                if v == true {
                    println!("Window {:?} was focused.", id);
                }
            }
            WindowEvent::CloseRequested => {
                println!("Application is now closing.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App {
        window: None,
        state: None,
    };
    let _ = event_loop.run_app(&mut app);
}
