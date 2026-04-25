use crate::{FOV, GAME_HEIGHT, GAME_WIDTH, helper};

pub struct Renderer {
    pub pixel_buffer: Vec<u32>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            pixel_buffer: vec![0u32; GAME_WIDTH * GAME_HEIGHT]
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < GAME_WIDTH as i32 && y >= 0 && y < GAME_HEIGHT as i32 {
            let offset = (y as usize) * GAME_WIDTH + (x as usize);
            self.pixel_buffer[offset] = color;
        }
    }

    pub fn clear(&mut self) {
        self.pixel_buffer.fill(helper::rgb(0, 0, 0));
    }

    pub fn as_u8_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.pixel_buffer.as_ptr() as *const u8,
                self.pixel_buffer.len() * 4,
            )
        }
    }
}