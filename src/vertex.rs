use crate::{FOV, GAME_HEIGHT, GAME_WIDTH};


pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn project(&self) -> (i32, i32) {
        let z_safe = if self.z == 0.0 { 0.1 } else { self.z };
        
        let x = (self.x / z_safe * FOV as f32) as i32 + (GAME_WIDTH / 2) as i32;
        let y = (self.y / z_safe * FOV as f32) as i32 + (GAME_HEIGHT / 2) as i32;
        (x, y)
    }
}