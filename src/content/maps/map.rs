use crate::utils::gpu::Vertex;

pub struct Map {
    pub position: [f32; 2],
    pub rotation: f32,
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[u32]>,
}

impl Map {
    pub fn new(new_vertices: Box<[Vertex]>, new_indices: Box<[u32]>) -> Self {
        Self {
            position: [0., 0.],
            rotation: 0.,
            vertices: new_vertices,
            indices: new_indices,
        }
    }
}