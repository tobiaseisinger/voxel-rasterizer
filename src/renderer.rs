use crate::{FOV, GAME_HEIGHT, GAME_WIDTH, helper, vertex::Vertex};

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

    pub fn draw_line(&mut self, v1: Vertex, v2: Vertex, color: u32) {
        let (x1, y1) = v1.project();
        let (x2, y2) = v2.project();

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;
        let mut curr_x = x1;
        let mut curr_y = y1;

        loop {
            self.draw_pixel(curr_x, curr_y, color);

            if curr_x == x2 && curr_y == y2 { break; }

            let e2 = 2 * err;
            
            if e2 >= dy {
                err += dy;
                curr_x += sx;
            }
            
            if e2 <= dx {
                err += dx;
                curr_y += sy;
            }
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