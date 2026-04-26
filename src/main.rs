extern crate sdl2;
mod vertex;
mod renderer;
mod helper;
mod block;

use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureAccess, TextureCreator};
use std::time::{Duration, Instant};
use vertex::Vertex;
use renderer::Renderer;
use block::Block;


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

    let mut renderer = Renderer::new();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_time = Instant::now();
    let mut total_time: f32 = 0.0;

    let mut vertices: Vec<Vertex> = Vec::new();

    vertices.push(Vertex::new(1.0, 1.0, 1.0));
    vertices.push(Vertex::new(0.0, 1.0, 2.0));

    let mut block = Block::new();

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
        renderer.clear();
    
        // drawing a vector
        let vec_color = helper::rgb(255, 255, 255);


        block.update(total_time);
        block.render(&mut renderer, vec_color);

        texture.update(None, renderer.as_u8_slice(), GAME_WIDTH * 4).unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

