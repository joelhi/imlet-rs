use implicit_engine::types::geometry::{Mesh, Vec3f};

use super::vertex::Vertex;

pub fn mesh_to_buffers(mesh: &Mesh) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices());
    let default = vec![
        Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0
        };
        mesh.num_vertices()
    ];
    let normals = mesh.get_normals().unwrap_or(&default);
    for (v, n) in mesh.get_vertices().iter().zip(normals) {
        vertices.push(Vertex {
            position: [v.x, v.y, v.z],
            normal: [n.x, n.y, n.z],
        })
    }

    let mut indices: Vec<u32> = Vec::with_capacity(mesh.num_faces() * 3);
    for face in mesh.get_faces() {
        indices.push(face[0] as u32);
        indices.push(face[1] as u32);
        indices.push(face[2] as u32);
    }

    (vertices, indices)
}