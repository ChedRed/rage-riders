use crate::utils::gpu::Vertex;

pub struct Car {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[u32]>,
}

impl Car {
    pub fn new(new_vertices: Box<[Vertex]>, new_indices: Box<[u32]>) -> Self {
        Self {
            vertices: new_vertices,
            indices: new_indices,
        }
    }
}