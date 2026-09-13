use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop};
use winit::window::Window;

#[derive(Default)]
struct App {
    window: Option<Box<dyn Window>>,
}

impl ApplicationHandler for App {

}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let event_loop = EventLoop::new().unwrap();

    event_loop.run_app(App::default())?;

    Ok(())
}
