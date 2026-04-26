use crate::{renderer::Renderer, vertex::Vertex};

pub struct Block {
    base_vertices: Vec<Vertex>,
    current_vertices: Vec<Vertex>,
    indices: Vec<(usize, usize)>
}

impl Block {
    pub fn new() -> Self {
        let mut vec:Vec<Vertex> = Vec::new();
        let mut indices: Vec<(usize, usize)> = Vec::new();

        let base = vec![
            Vertex::new(-1.0, -1.0, -1.0), Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),  Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),  Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),   Vertex::new(-1.0, 1.0, 1.0),
        ];

        let indices = vec![
            (0, 1), (1, 2), (2, 3), (3, 0),
            (4, 5), (5, 6), (6, 7), (7, 4),
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];

        Self {
            current_vertices: base.clone(),
            base_vertices: base,
            indices,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, color: u32) {
        for (start, end) in &self.indices {
            let v1 = &self.current_vertices[*start];
            let v2 = &self.current_vertices[*end];

            renderer.draw_line(v1.clone(), v2.clone(), color);
        }
    }
    
    pub fn update(&mut self, t: f32) {
        for (i, base) in self.base_vertices.iter().enumerate() {
            let x = base.x * t.cos() - base.z * t.sin();
            let z = base.x * t.sin() + base.z * t.cos();
            let y = base.y;

            self.current_vertices[i] = Vertex::new(x, y, z + 5.0);
        }
    }
}