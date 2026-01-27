use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};


struct WindowState {
    window: &'static Window,
    pixels: Pixels<'static>,
}

#[derive(Default)]
pub struct App {
    window_state: Option<WindowState>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let  window_attributes = Window::default_attributes()
            .with_title("Voxel Rasterizer")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 480.0))
            .with_resizable(false);

        let window = event_loop.create_window(window_attributes).unwrap();

        let window:&'static Window = Box::leak(Box::new(window));

        let surface_texture = SurfaceTexture::new(640, 480, window);

        let pixels = Pixels::new(640, 480, surface_texture).expect("Failed to create pixels");

        self.window_state = Some(WindowState {window, pixels});
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(state) = &mut self.window_state {
            match event {
                WindowEvent::CloseRequested => {
                    println!("Closing window!");
                    event_loop.exit();
                },
                WindowEvent::RedrawRequested => {
                    let frame = state.pixels.frame_mut();

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel[0] = 0x00;
                        pixel[1] = 0x00;
                        pixel[2] = 0xFF;
                        pixel[3] = 0xFF;
                    }

                    state.pixels.render().unwrap();

                    state.window.request_redraw();
                },
                _ => (),
            }
        }
    }
}