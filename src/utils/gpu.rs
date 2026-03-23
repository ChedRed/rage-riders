use assert_json_diff::assert_json_include;
use serde_json::json;

#[repr(C)]
#[derive(Copy, Clone, Debug, serde::Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Location {
    pub position: [f32; 2],
    pub rotation: [f32; 2],
}

impl Location {
    pub fn new() -> Self {
        Self {
            position: [0., 0.],
            rotation: [0., 0.],
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Location>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

macro_rules! include_object {
    ($path:expr) => {{
        let vectors = include_bytes!("vectors/data_schema.json");
    }};
}

macro_rules! include_vectors {
    ($path:expr) => {{
        let vectors = include_str!($path);
        let schema = include_str!("vectors/data_schema.json");
        
        let vector_json: serde_json::Value = serde_json::from_str(vectors).unwrap();
        let json_schema: serde_json::Value = serde_json::from_str(schema).unwrap();
        
        assert_json_diff::assert_json_include!(actual: vector_json, expected: json_schema);
        
        let vertices: Vec<Vertex> = serde_json::from_value(vector_json["vertices"].clone()).unwrap();
        let indices: Vec<u32> = serde_json::from_value(vector_json["indices"].clone()).unwrap();
        
        let box_vertices: Box<[Vertex]> = vertices.into_boxed_slice();
        let box_indices: Box<[u32]> = indices.into_boxed_slice();
        
        (box_vertices, box_indices)
    }};
}
pub(crate) use include_vectors;