extern crate sdl2;

use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureAccess, TextureCreator};
use std::time::{Duration, Instant};

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const GAME_WIDTH: usize = 320;
const GAME_HEIGHT: usize = 240;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, GAME_WIDTH as u32, GAME_HEIGHT as u32)
        .unwrap();

    // Pixel Buffer with the size of WIDTH * HEIGHT
    let mut pixel_buffer = vec![0u32; GAME_WIDTH * GAME_HEIGHT];
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_time = Instant::now();
    let mut total_time: f32 = 0.0;

    // game loop
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        total_time += delta_time;

        // event handeling for inputs and stuff
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        

        // loop for filling the buffer
        for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                let offset = y * GAME_WIDTH + x;
                
                let r = (x % 255) as u32;
                let g = (y % 255) as u32;
                let b = ((total_time.sin() * 0.5 + 0.5) * 255.0) as u32;

                let color = rgb(r, g, b);

                pixel_buffer[offset] = color;
            }
        }

        let u8_slice = unsafe {
            std::slice::from_raw_parts(
                pixel_buffer.as_ptr() as *const u8,
                pixel_buffer.len() * 4
            )
        };

        texture.update(None, u8_slice, GAME_WIDTH * 4).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


fn rgb(r: u32, g: u32, b: u32) -> u32 {
    0xFF000000 | (r << 16) | (g << 8) | b
}