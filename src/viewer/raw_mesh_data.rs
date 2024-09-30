use bevy::prelude::Resource;

use crate::types::geometry::{BoundingBox, Mesh, Vec3};

use super::utils::{convert_vec3_to_raw, faces_as_flat_u32};

#[derive(Debug, Clone, Resource)]
pub struct RawMeshData {
    pub vertex_data: Vec<[f32; 3]>,
    pub faces: Vec<u32>,
    pub bounds: BoundingBox<f32>,
}

impl RawMeshData {
    pub fn from_mesh(mesh: &Mesh<f32>) -> Self {
        Self {
            vertex_data: convert_vec3_to_raw(mesh.vertices()),
            faces: faces_as_flat_u32(mesh.faces()),
            bounds: mesh.bounds(),
        }
    }

    pub fn centroid(&self) -> Vec3<f32> {
        self.bounds.centroid()
    }
}
