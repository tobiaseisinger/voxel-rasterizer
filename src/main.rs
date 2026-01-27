mod apphandler;

use winit::event_loop::{ControlFlow, EventLoop};
use crate::apphandler::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("PANIC! Can't start the application");
}
