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
    pub rotation: [f32; 4],
}

impl Location {
    pub fn new() -> Self {
        Self {
            position: [0., 0.],
            rotation: [0., 0., 0., 0.],
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
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

macro_rules! include_object {
    ($path:expr) => {{
        let mut offset: usize = 0;
        
        static BYTES: &[u8] = include_bytes!($path);
        
        let mut center: [f32; 2] = [0., 0.];
        center[0] = f32::from_be_bytes(
            BYTES[offset..offset+4].try_into().expect("Invalid file! Are you sure it has been filled out properly?")
        );
        offset += 4;
        center[1] = f32::from_be_bytes(
            BYTES[offset..offset+4].try_into().expect("Invalid file! Are you sure it has been filled out properly?")
        );
        offset += 4;
        
        let mut count: usize = u32::from_be_bytes(
            BYTES[offset..offset+4].try_into().expect("Invalid file! Are you sure it has been filled out properly?")
        ) as usize;
        offset += 4;
        
        let mut pre_indices: Vec<u32> = Vec::with_capacity(count);
        
        for _ in 0..count {
            let chunk = &BYTES[offset..offset+4];
            pre_indices.push(u32::from_be_bytes(chunk.try_into().unwrap()));
            offset += 4;
        }
        
        count = u32::from_be_bytes(
            BYTES[offset..offset+4].try_into().expect("Invalid file! Are you sure it has been filled out properly?")
        ) as usize;
        offset += 4;
        
        let mut pre_vertices: Vec<Vertex> = Vec::with_capacity(count);
        
        for _ in 0..count {
            pre_vertices.push(Vertex {
                position: [
                    f32::from_be_bytes(BYTES[offset..offset+4].try_into().unwrap()),
                    f32::from_be_bytes(BYTES[offset+4..offset+8].try_into().unwrap()),
                ],
                color: [
                    f32::from_be_bytes(BYTES[offset+8..offset+12].try_into().unwrap()),
                    f32::from_be_bytes(BYTES[offset+12..offset+16].try_into().unwrap()),
                    f32::from_be_bytes(BYTES[offset+16..offset+20].try_into().unwrap()),
                    f32::from_be_bytes(BYTES[offset+20..offset+24].try_into().unwrap()),    
                ],
            });
            offset += 24;
        }
        
        (pre_vertices.into_boxed_slice(), pre_indices.into_boxed_slice(), center)
    }};
}

pub(crate) use include_object;