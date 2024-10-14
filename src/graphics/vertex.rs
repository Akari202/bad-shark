use vec_utils::vec3d::Vec3d;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Vertex {
    pub fn from_vec3d(vector: &Vec3d, color: [f32; 3]) -> Self {
        Self {
            position: [
                vector.x as f32,
                vector.y as f32,
                vector.z as f32
            ],
            color
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Self {
            position: [self.position[0] / scale, self.position[1] / scale, self.position[2] / scale],
            color: self.color
        }
    }

    pub fn mirror(&self) -> Self {
        Self {
            position: [self.position[0], self.position[1] * -1.0, self.position[2]],
            color: self.color
        }
    }

    // pub fn from_vec3ds(vectors: &[&Vec3d], color: [f32; 3]) -> Vec<Self> {
    //     vectors.iter().map(|i| {Self::from_vec3d(i, color)}).collect()
    // }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}
