// Module responsible for handling the window and rendering to it
use std::rc::Rc;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::{Window, WindowId, WindowAttributes};
use softbuffer::{Context, Surface};

pub mod camera;
pub mod canvas;

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

        // self.canvas = canvas::Canvas::default();
        self.canvas = canvas::Canvas::new(5, 5, 0xAA000000);
        self.canvas.fill(0x00777777);
        self.canvas.dither(0x00FFFFFF, 2);
        self.canvas.scale_to(6, 6);
        self.canvas.scale_to(120, 120);
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
