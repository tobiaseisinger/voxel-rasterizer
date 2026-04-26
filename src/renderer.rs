use sdl2::render;

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

    pub fn draw_filled_triangle(&mut self, mut v1: (i32, i32), mut v2: (i32, i32), mut v3: (i32, i32), color: u32) {
        let mut x_left:Vec<i32> = Vec::new();
        let mut x_right:Vec<i32> = Vec::new();

        let mut vertices = vec![
            v1, v2, v3
        ];

        vertices.sort_by(|v1, v2| v1.1.cmp(&v2.1));

        let p0 = vertices[0];
        let p1 = vertices[1];
        let p2 = vertices[2];

        let x01 = helper::interpolate(p0.1, p0.0, p1.1, p1.0);
        let x12 = helper::interpolate(p1.1, p1.0, p2.1, p2.0);
        let x02 = helper::interpolate(p0.1, p0.0, p2.1, p2.0);

        let mut x012 = x01;
        x012.pop();
        x012.extend(x12);

        let m = x02.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012
        } else {
            x_left = x012;
            x_right = x02;
        }

        for (i, y) in (p0.1..=p2.1).enumerate() {
            let start_x = x_left[i];
            let end_x = x_right[i];
            
            for x in start_x..=end_x {
                self.draw_pixel(x, y, color);
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