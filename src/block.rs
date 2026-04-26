use crate::{renderer::Renderer, vertex::Vertex};

pub struct Block {
    base_vertices: Vec<Vertex>,
    current_vertices: Vec<Vertex>,
    indices: Vec<[usize; 3]>
}

impl Block {
    pub fn new() -> Self {
        let mut vec:Vec<Vertex> = Vec::new();
        let mut indices: Vec<[usize; 3]> = Vec::new();

        let base = vec![
            Vertex::new(-1.0, -1.0, -1.0), Vertex::new(1.0, -1.0, -1.0),
            Vertex::new(1.0, 1.0, -1.0),  Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, 1.0),  Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, 1.0),   Vertex::new(-1.0, 1.0, 1.0),
        ];

        let indices = vec![
            // Front
            [0, 1, 2], [0, 2, 3],
            // Back
            [4, 5, 6], [4, 6, 7],
            // Left
            [0, 3, 7], [0, 7, 4],
            // Right
            [1, 2, 6], [1, 6, 5],
            // Top
            [3, 2, 6], [3, 6, 7],
            // Bottom
            [0, 1, 5], [0, 5, 4]
        ];

        Self {
            current_vertices: base.clone(),
            base_vertices: base,
            indices: indices,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, color: u32) {
        for face in &self.indices {
            let v1 = &self.current_vertices[face[0]];
            let v2 = &self.current_vertices[face[1]];
            let v3 = &self.current_vertices[face[2]];

            let p1 = v1.project();
            let p2 = v2.project();
            let p3 = v3.project();

            renderer.draw_filled_triangle(p1, p2, p3, color);
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