use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use super::SpatialHashGrid;
use super::XYZ;
use std::time::Instant;
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

    pub fn get_normals(&self) -> Option<&Vec<XYZ>> {
        self.normals.as_ref()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn get_centroid(&self) -> XYZ {
        let mut centroid: XYZ = XYZ::origin();

        for &v in self.get_vertices() {
            centroid = centroid + v;
        }

        centroid * (1.0 / self.num_vertices() as f32)
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

    pub fn compute_vertex_normals(&mut self) {
        let face_normals: Vec<XYZ> = self.compute_face_normals();
        let vertex_faces: Vec<Vec<usize>> = self.compute_vertex_faces();
        let mut vertex_normals = vec![XYZ::origin(); self.num_vertices()];
        vertex_normals
            .par_iter_mut()
            .enumerate()
            .for_each(|(id, n)| {
                for &f in &vertex_faces[id] {
                    *n = *n + face_normals[f];
                }
                *n = *n / (vertex_faces[id].len() as f32)
            });
        self.normals = Some(vertex_normals);
    }

    pub fn compute_face_normals(&self) -> Vec<XYZ> {
        self.faces
            .par_iter()
            .map(|f| {
                let v1 = self.vertices[f[1]] - self.vertices[f[0]];
                let v2 = self.vertices[f[2]] - self.vertices[f[0]];
                v1.cross(v2).normalize()
            })
            .collect()
    }

    pub fn compute_vertex_faces(&self)->Vec<Vec<usize>>{
        let mut vertex_faces = vec![Vec::with_capacity(16); self.num_vertices()];
        self.faces.iter().enumerate().for_each(|(id, f)| {
            vertex_faces[f[0]].push(id);
            vertex_faces[f[1]].push(id);
            vertex_faces[f[2]].push(id);
        });
        vertex_faces
    }

    pub fn from_triangles(triangles: &[Triangle]) -> Mesh {
        let before = Instant::now();
        // Contruct vertex buffer using a hash grid for coordinates to index mapping
        let mut faces: Vec<[usize; 3]> = Vec::new();
        let mut grid = SpatialHashGrid::new();

        let mut mesh = Mesh::new();
        for triangle in triangles {
            let vertex_ids = [
                grid.add_point(triangle.p1),
                grid.add_point(triangle.p2),
                grid.add_point(triangle.p3),
            ];

            if !(vertex_ids[0] == vertex_ids[1]
                || vertex_ids[0] == vertex_ids[2]
                || vertex_ids[1] == vertex_ids[2])
            {
                faces.push(vertex_ids);
            }
        }

        mesh.add_vertices(&grid.vertices());
        mesh.add_faces(&faces);

        log::info!(
            "Mesh topology generated for {} points and {} triangles in {:.2?}",
            mesh.num_vertices(),
            mesh.num_faces(),
            before.elapsed()
        );
        let before = Instant::now();
        mesh.compute_vertex_normals();

        log::info!(
            "Mesh normals computed for {} points in {:.2?}",
            mesh.num_vertices(),
            before.elapsed()
        );

        mesh
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: XYZ,
    pub p2: XYZ,
    pub p3: XYZ,
}
