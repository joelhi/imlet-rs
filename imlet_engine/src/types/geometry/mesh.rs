use num_traits::Float;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use super::BoundingBox;
use super::SpatialHashGrid;
use super::Vec3;
use std::fmt::Debug;
use std::time::Instant;
use std::usize;

pub struct Mesh<T: Float + Debug> {
    vertices: Vec<Vec3<T>>,
    faces: Vec<[usize; 3]>,
    normals: Option<Vec<Vec3<T>>>,
}

impl<T: Float + Debug> Mesh<T> {
    pub fn new() -> Mesh<T> {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: None,
        }
    }

    pub fn from_triangles(triangles: &[Triangle<T>]) -> Mesh<T> {
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

    pub fn add_vertices(&mut self, vertices: &[Vec3<T>]) {
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_faces(&mut self, faces: &[[usize; 3]]) {
        self.faces.extend_from_slice(faces);
    }

    pub fn get_vertices(&self) -> &Vec<Vec3<T>> {
        &self.vertices
    }

    pub fn get_faces(&self) -> &Vec<[usize; 3]> {
        &self.faces
    }

    pub fn get_normals(&self) -> Option<&Vec<Vec3<T>>> {
        self.normals.as_ref()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn get_centroid(&self) -> Vec3<T> {
        let mut centroid: Vec3<T> = Vec3::origin();

        for &v in self.get_vertices() {
            centroid = centroid + v;
        }

        centroid * (1.0 / self.num_vertices() as T)
    }

    pub fn get_bounds(&self) -> BoundingBox<T> {
        let mut max = Vec3::new(-Float::max_value(), -Float::max_value(), -Float::max_value());
        let mut min = Vec3::new(Float::max_value(), Float::max_value(), Float::max_value());

        for v in self.get_vertices() {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);

            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        BoundingBox::new(min, max)
    }

    pub fn compute_vertex_normals(&mut self) {
        let face_normals: Vec<Vec3<T>> = self.compute_face_normals();
        let vertex_faces: Vec<Vec<usize>> = self.compute_vertex_faces();
        let mut vertex_normals = vec![Vec3::origin(); self.num_vertices()];
        vertex_normals
            .par_iter_mut()
            .enumerate()
            .for_each(|(id, n)| {
                for &f in &vertex_faces[id] {
                    *n = *n + face_normals[f];
                }
                *n = *n / (vertex_faces[id].len() as T)
            });
        self.normals = Some(vertex_normals);
    }

    pub fn compute_face_normals(&self) -> Vec<Vec3<T>> {
        self.faces
            .par_iter()
            .map(|f| {
                let v1 = self.vertices[f[1]] - self.vertices[f[0]];
                let v2 = self.vertices[f[2]] - self.vertices[f[0]];
                v1.cross(v2).normalize()
            })
            .collect()
    }

    pub fn compute_vertex_faces(&self) -> Vec<Vec<usize>> {
        let mut vertex_faces = vec![Vec::with_capacity(16); self.num_vertices()];
        self.faces.iter().enumerate().for_each(|(id, f)| {
            vertex_faces[f[0]].push(id);
            vertex_faces[f[1]].push(id);
            vertex_faces[f[2]].push(id);
        });
        vertex_faces
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle<T: Float + Debug> {
    pub p1: Vec3<T>,
    pub p2: Vec3<T>,
    pub p3: Vec3<T>,
}

impl<T: Float + Debug> Triangle<T> {
    pub fn compute_area(&self) -> T {
        let a = self.p1.distance_to_vec3f(self.p2);
        let b = self.p2.distance_to_vec3f(self.p3);
        let c = self.p3.distance_to_vec3f(self.p1);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}
