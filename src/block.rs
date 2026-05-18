use crate::{helper, renderer::Renderer, vertex::Vertex};

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
            // Front (Z = -1)
            [0, 1, 2], [0, 2, 3],
            // Back (Z = 1)
            [4, 6, 5], [4, 7, 6],
            // Left (X = -1)
            [0, 5, 1], [0, 4, 5],
            // Right (X = 1)
            [2, 7, 3], [2, 6, 7],
            // Top (Y = 1)
            [3, 4, 0], [3, 7, 4],
            // Bottom (Y = -1)
            [1, 6, 2], [1, 5, 6]
        ];

        Self {
            current_vertices: base.clone(),
            base_vertices: base,
            indices: indices,
        }
    }

    pub fn render(&self, renderer: &mut Renderer, color: u32) {
        let mut visible_faces: Vec<(f32, [usize; 3], Vertex)> = Vec::new();

        let light = Vertex::new(0.0, 0.0, 0.5);
        let len = (light.x*light.x + light.y*light.y + light.z*light.z).sqrt();
        let light = Vertex::new(light.x/len, light.y/len, light.z/len);

        for face in &self.indices {
            let v1 = &self.current_vertices[face[0]];
            let v2 = &self.current_vertices[face[1]];
            let v3 = &self.current_vertices[face[2]];

            if v1.z <= 0.0 || v2.z <= 0.0 || v3.z <= 0.0 { continue; }

            let a = Vertex::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
            let b = Vertex::new(v3.x - v1.x, v3.y - v1.y, v3.z - v1.z);
            let normal_vec = Vertex::cross(a, b);

            let dot = normal_vec.x * v1.x + normal_vec.y * v1.y + normal_vec.z * v1.z;            

            if dot > 0.0 {
                let avg_z = (v1.z + v2.z + v3.z) / 3.0;
                let len = (normal_vec.x*normal_vec.x + normal_vec.y*normal_vec.y + normal_vec.z +normal_vec.z).sqrt();
                let normal = Vertex::new(normal_vec.x/len, normal_vec.y/len, normal_vec.z/len);
                visible_faces.push((avg_z, *face, normal));
            }
        }

        visible_faces.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        for (_, face, normal) in &visible_faces {
            let v1 = &self.current_vertices[face[0]];
            let v2 = &self.current_vertices[face[1]];
            let v3 = &self.current_vertices[face[2]];

            let dot_light = (normal.x*light.x + normal.y*light.y + normal.z*light.z).max(0.0);
            let intensity = 0.15 + 0.85 * dot_light;
            let shaded = helper::apply_intensity(color, intensity);

            renderer.draw_filled_triangle(v1.project(), v2.project(), v3.project(), shaded);
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