use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId, WindowAttributes};

#[derive(Default)]
struct App {
    window: Option<Box<dyn Window>>,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
    ) {
        self.window = Some(event_loop.create_window(WindowAttributes::default()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window Closed via Request");
                event_loop.exit();
            },
            WindowEvent::Focused(state) => {
                println!("Window focus is now {state}");
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                println!("Keyboard event: {:?}", event);
            },
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(App::default())?;

    Ok(())
}
