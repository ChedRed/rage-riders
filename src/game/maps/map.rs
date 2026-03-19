use crate::utils::gpu::{Vertex, Location};

pub struct Map<'a> {
    pub position: [f32; 2],
    pub rotation: f32,
    pub vertices: &'a[Vertex],
    pub indices: &'a[Location],
}

impl Map {
    pub fn load(new_vertices: &[Vertex], new_indices: &[Vertex]) {
        vertices = new_vertices;
        indices = new_indices;
    }
}