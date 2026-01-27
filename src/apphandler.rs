use winit::application::ApplicationHandler;
use winit::dpi::LogicalPosition;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let  window_attributes = Window::default_attributes()
            .with_title("Voxel Rasterizer")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
            .with_resizable(false);

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window!");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // draw methods

                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }
    }
}