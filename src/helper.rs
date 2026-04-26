pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn interpolate(y0: i32, x0: i32, y1: i32, x1: i32) -> Vec<i32> {
    let mut values = Vec::new();
    if y0 == y1 {
        values.push(x0);
        return values;
    }

    let a = (x1 - x0) as f32 / (y1 - y0) as f32;
    let mut x = x0 as f32;

    for _ in y0..=y1 {
        values.push(x as i32);
        x += a;
    }
    values
}