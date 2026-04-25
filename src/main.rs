extern crate sdl2;
mod vertex;

use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureAccess, TextureCreator};
use std::time::{Duration, Instant};
use vertex::Vertex;


const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const GAME_WIDTH: usize = 320;
const GAME_HEIGHT: usize = 240;
const FOV: usize = 70;


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

    let mut vertices: Vec<Vertex> = Vec::new();

    vertices.push(Vertex::new(1.0, 1.0, 1.0));

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

        // clearing the buffer
        pixel_buffer.fill(rgb(0, 0, 0));
    
        // drawing a vector
        let vec_color = rgb(255, 255, 255);

        vertices[0].x = (total_time.cos() * 2.0);
        vertices[0].y = (total_time.sin() * 2.0);
        vertices[0].z = 5.0;

        for v in vertices.iter_mut() {
            let (v_x, v_y) = v.project();

            if v_x >= 0 && v_x < GAME_WIDTH as i32 && v_y >= 0 && v_y < GAME_HEIGHT as i32 {
                let offset = (v_y as usize) * GAME_WIDTH + (v_x as usize);
                pixel_buffer[offset] = vec_color;
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