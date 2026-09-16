// Module responsible for handling the window and rendering to it
use std::rc::Rc;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId, WindowAttributes};
use softbuffer::{Context, Surface};

mod camera;
mod canvas;

#[derive(Default)]
pub struct App {
    window: Option<Rc<dyn Window>>,
    context: Option<Context<Rc<dyn Window>>>,
    surface: Option<Surface<Rc<dyn Window>, Rc<dyn Window>>>,

    canvas: canvas::Canvas,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
    ) {
        let window: Rc<dyn Window> = Rc::from(event_loop.create_window(WindowAttributes::default()).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.context = Some(context);
        self.surface = Some(surface);
        self.window = Some(window);

        self.window.as_ref().unwrap().request_redraw();

        self.canvas = canvas::Canvas::default();
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
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().surface_size();
                    (size.width, size.height)
                };

                let surface = self.surface.as_mut().unwrap();
                surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                buffer.copy_from_slice(&self.canvas.canvas_to_screen(width, height));

                // for pixel in buffer.iter_mut() {
                //     *pixel = 0x00000000;
                // }

                buffer.present().unwrap();

                // self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::KeyboardInput {event, .. } => {
                println!("Keyboard event: {:?}", event);
            },
            _ => (),
        }
    }
}

pub fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();

    // event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(App::default())?;

    Ok(())
}
