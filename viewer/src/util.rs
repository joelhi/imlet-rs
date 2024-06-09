use std::fmt::Debug;

use cgmath::num_traits::Float;
use imlet_engine::types::geometry::{Line, Mesh, Vec3};

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

pub fn lines_to_buffer<T: Float + Debug + Send + Sync>(lines: &[Line<T>])->Vec<Vertex>{
    let mut vertices: Vec<Vertex> = Vec::with_capacity(2*lines.len());
    for line in lines{
        vertices.push(Vertex::from_vec3(&line.start, &Vec3::origin()));
        vertices.push(Vertex::from_vec3(&line.end, &Vec3::origin()));
    }
    return vertices;
}
