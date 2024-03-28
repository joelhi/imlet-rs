use std::{collections::HashMap, usize};
use crate::engine::types::hash_grid::SpatialHashGrid;
use super::core::XYZ;



pub struct Mesh{
    vertices:Vec<XYZ>,
    faces:Vec<[usize; 3]>,
}

impl Mesh {
    pub fn new()->Mesh{
        Mesh{
            vertices:Vec::new(),
            faces:Vec::new()
        }
    }

    pub fn add_vertices(&mut self, vertices: &[XYZ]){
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_faces(&mut self, faces: &[[usize; 3]]){
        self.faces.extend_from_slice(faces);
    }

    pub fn get_vertices(&self)->&Vec<XYZ>{
        &self.vertices
    }

    pub fn get_faces(&self)->&Vec<[usize; 3]>{
        &self.faces
    }

    pub fn from_triangles(triangles:&[Triangle])->Mesh{
        // Contruct vertex buffer using a hashset for coordinates to index mapping
        let mut faces:Vec<[usize;3]> = Vec::new();
        let mut grid = SpatialHashGrid::new();

        let mut mesh = Mesh::new();
        for triangle in triangles{
            faces.push(
                [
                    grid.add_point(triangle.p1),
                    grid.add_point(triangle.p2),
                    grid.add_point(triangle.p3)
                ]
            );
        }
        mesh.add_vertices(&grid.vertices());
        mesh.add_faces(&faces);
        mesh
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: XYZ,
    pub p2: XYZ,
    pub p3: XYZ,
}