use std::fmt::Debug;

use cgmath::num_traits::Float;
use imlet_engine::types::geometry::{Mesh, Vec3};

use super::vertex::Vertex;

pub fn mesh_to_buffers<T: Float + Debug + Send + Sync>(mesh: &Mesh<T>) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices());
    let default = &vec![
        Vec3 {
            x: T::one(),
            y: T::one(),
            z: T::one()
        };
        mesh.num_vertices()
    ];
    let normals = mesh.get_normals().unwrap_or(default);
    for (v, n) in mesh.get_vertices().iter().zip(normals) {
        vertices.push(Vertex::from_vec3(v, n))
    }

    let mut indices: Vec<u32> = Vec::with_capacity(mesh.num_faces() * 3);
    for face in mesh.get_faces() {
        indices.push(face[0] as u32);
        indices.push(face[1] as u32);
        indices.push(face[2] as u32);
    }

    (vertices, indices)
}
