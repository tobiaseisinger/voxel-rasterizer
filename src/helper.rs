pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn interpolate(y0: i32, v0: i32, y1: i32, v1: i32) -> Vec<i32> {
    let mut values = Vec::new();
    if y0 == y1 {
        values.push(v0);
        return values;
    }

    let num_steps = y1 - y0; 
    let dv = (v1 - v0) as f32;
    let dy = (y1 - y0) as f32;

    for i in 0..=num_steps {
        let t = i as f32 / dy;
        let v = v0 as f32 + (t * dv);
        
        values.push(v.round() as i32);
    }
    values
}

pub fn apply_intensity(color: u32, intensity: f32) -> u32 {
    let r = ((color >> 16) & 0xFF) as f32 * intensity;
    let g = ((color >> 8)  & 0xFF) as f32 * intensity;
    let b =  (color        & 0xFF) as f32 * intensity;
    rgb(r as u8, g as u8, b as u8)
}