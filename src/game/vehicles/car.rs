use crate::utils::gpu::{Vertex, Location};

pub struct Car <'a> {
    pub position: [f32; 2],
    pub rotation: f32,
    pub vertices: &'a[Vertex],
    pub indices: &'a[Location],
}
