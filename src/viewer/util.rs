use crate::types::geometry::{Line, Mesh, Vec3};
use cgmath::num_traits::Float;

use super::vertex::Vertex;

const MAX_LINE_BUFFER_SIZE: usize = 65000000;

pub fn mesh_to_buffers<T: Float>(mesh: &Mesh<T>) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(mesh.num_vertices());
    let default = &vec![
        Vec3 {
            x: T::one(),
            y: T::one(),
            z: T::one()
        };
        mesh.num_vertices()
    ];
    let normals = mesh.normals().unwrap_or(default);
    for (v, n) in mesh.vertices().iter().zip(normals) {
        vertices.push(Vertex::from_vec3(v, n))
    }

    let mut indices: Vec<u32> = Vec::with_capacity(mesh.num_faces() * 3);
    for face in mesh.faces() {
        indices.push(face[0] as u32);
        indices.push(face[1] as u32);
        indices.push(face[2] as u32);
    }

    (vertices, indices)
}

pub fn lines_to_buffer<T: Float>(lines: &[Line<T>]) -> Vec<Vec<Vertex>> {
    let mut output: Vec<Vec<Vertex>> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::with_capacity(MAX_LINE_BUFFER_SIZE);
    for line in lines {
        if vertices.len() >= MAX_LINE_BUFFER_SIZE {
            output.push(vertices);
            vertices = Vec::with_capacity(MAX_LINE_BUFFER_SIZE);
        }

        vertices.push(Vertex::from_vec3(&line.start, &Vec3::origin()));
        vertices.push(Vertex::from_vec3(&line.end, &Vec3::origin()));
    }
    output.push(vertices);
    return output;
}
