use std::{collections::HashMap, usize};

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
        let mut vertices:Vec<XYZ> = Vec::new();
        let mut faces:Vec<[usize;3]> = Vec::new();
        let mut map: HashMap<usize, usize> = HashMap::new();

        let mut mesh = Mesh::new();
        for triangle in triangles{
            faces.push(
                [
                    Self::resolve_vertex_index(
                        triangle.p1,
                        &mut map,
                        &mut vertices
                    ),
                    Self::resolve_vertex_index(
                        triangle.p2,
                        &mut map,
                        &mut vertices
                    ),
                    Self::resolve_vertex_index(
                        triangle.p3,
                        &mut map,
                        &mut vertices
                    )
                ]
            );
        }
        mesh.add_vertices(&vertices);
        mesh.add_faces(&faces);
        mesh
    }

    pub fn resolve_vertex_index(pt: XYZ, map:&mut HashMap<usize,usize>, vertices: &mut Vec<XYZ>)->usize{
        let index: usize;
        let hash = &pt.spatial_hash();
        if map.contains_key(hash){
            index = map[hash];
        }
        else{
            index = vertices.len();
            map.insert(*hash, index);
            vertices.push(pt);
        }

        index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: XYZ,
    pub p2: XYZ,
    pub p3: XYZ,
}