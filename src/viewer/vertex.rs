use std::fmt::Debug;

use crate::types::geometry::Vec3;
use num_traits::Float;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn from_vec3<T: Float>(position: &Vec3<T>, normal: &Vec3<T>) -> Self {
        Self {
            position: [
                position.x.to_f32().unwrap(),
                position.z.to_f32().unwrap(),
                position.y.to_f32().unwrap(),
            ],
            normal: [
                normal.x.to_f32().unwrap(),
                normal.z.to_f32().unwrap(),
                normal.y.to_f32().unwrap(),
            ],
        }
    }

    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}
