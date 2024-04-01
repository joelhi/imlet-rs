use super::SpatialHashGrid;
use super::XYZ;
use std::usize;

pub struct Mesh {
    vertices: Vec<XYZ>,
    faces: Vec<[usize; 3]>,
    normals: Option<Vec<XYZ>>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: None,
        }
    }

    pub fn add_vertices(&mut self, vertices: &[XYZ]) {
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_faces(&mut self, faces: &[[usize; 3]]) {
        self.faces.extend_from_slice(faces);
    }

    pub fn get_vertices(&self) -> &Vec<XYZ> {
        &self.vertices
    }

    pub fn get_faces(&self) -> &Vec<[usize; 3]> {
        &self.faces
    }

    pub fn num_vertices(&self)->usize{
        self.vertices.len()
    }

    pub fn num_faces(&self)->usize{
        self.faces.len()
    }

    pub fn get_centroid(&self)->XYZ{
        let mut centroid: XYZ = XYZ::origin();

        for &v in self.get_vertices(){
            centroid=centroid+v;
        }

        centroid*(1.0/self.num_vertices() as f32)
    }

    pub fn get_bounds(&self) -> (XYZ, XYZ) {
        let mut min = XYZ::origin();
        let mut max = XYZ::origin();
    
        for v in self.get_vertices() {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }
    
        (min, max)
    }

    pub fn compute_normals(&self){
        panic!("Not implemented.")
    }

    pub fn from_triangles(triangles: &[Triangle]) -> Mesh {
        // Contruct vertex buffer using a hash grid for coordinates to index mapping
        let mut faces: Vec<[usize; 3]> = Vec::new();
        let mut grid = SpatialHashGrid::new();

        let mut mesh = Mesh::new();
        for triangle in triangles {
            faces.push([
                grid.add_point(triangle.p1),
                grid.add_point(triangle.p2),
                grid.add_point(triangle.p3),
            ]);
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
