// Dependencies
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, window::Window
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
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Focused(v) => {
                if v == true {
                    println!("Window was focused.");
                }
            },
            WindowEvent::CloseRequested => {
                println!("Application is now closing.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => ()
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        window: None,
        state: None,
    };
    let _ = event_loop.run_app(&mut app);
}
