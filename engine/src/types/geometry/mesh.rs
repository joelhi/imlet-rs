use num_traits::Float;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use super::BoundingBox;
use super::Line;
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

impl<T: Float + Debug + Send + Sync> Mesh<T> {
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

    pub fn edges(&self) -> Vec<Line<T>> {
        let mut edges: Vec<Line<T>> = Vec::with_capacity(self.num_faces());
        for f in self.faces.iter() {
            edges.push(Line::new(self.vertices[f[0]], self.vertices[f[1]]));
            edges.push(Line::new(self.vertices[f[1]], self.vertices[f[2]]));
            edges.push(Line::new(self.vertices[f[2]], self.vertices[f[0]]));
        }
        edges
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

        centroid * T::from(1.0 / self.num_vertices() as f64).expect("Failed to convert number to T")
    }

    pub fn get_bounds(&self) -> BoundingBox<T> {
        let mut max = Vec3::new(-T::max_value(), -T::max_value(), -T::max_value());
        let mut min = Vec3::new(T::max_value(), T::max_value(), T::max_value());

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
                *n = *n
                    * T::from(1.0 / vertex_faces[id].len() as f64)
                        .expect("Failed to convert number to T");
            });
        self.normals = Some(vertex_normals);
    }

    pub fn compute_face_normals(&self) -> Vec<Vec3<T>> {
        self.faces
            .par_iter()
            .map(|f| {
                let v1 = self.vertices[f[1]] - self.vertices[f[0]];
                let v2 = self.vertices[f[2]] - self.vertices[f[0]];
                v1.cross(&v2).normalize()
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

    pub fn to_f32(&self) -> Mesh<f32> {
        let converted_v: Vec<Vec3<f32>> = self.vertices.iter().map(|v| v.to_f32()).collect();
        let mut m = Mesh::<f32>::new();

        m.add_vertices(&converted_v);
        m.add_faces(&self.faces);
        m.compute_vertex_normals();

        m
    }

    pub fn as_triangles(&self) -> Vec<Triangle<T>> {
        let mut triangles: Vec<Triangle<T>> = Vec::with_capacity(self.num_faces());
        for face in self.faces.iter() {
            triangles.push(Triangle::new(
                self.vertices[face[0]],
                self.vertices[face[1]],
                self.vertices[face[2]],
            ))
        }
        triangles
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle<T: Float + Debug> {
    pub p1: Vec3<T>,
    pub p2: Vec3<T>,
    pub p3: Vec3<T>,
}

impl<T: Float + Debug> Triangle<T> {
    pub fn new(p1: Vec3<T>, p2: Vec3<T>, p3: Vec3<T>) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn ZERO() -> Self {
        Self {
            p1: Vec3::origin(),
            p2: Vec3::origin(),
            p3: Vec3::origin(),
        }
    }

    pub fn compute_area(&self) -> T {
        let a = self.p1.distance_to_vec3(&self.p2);
        let b = self.p2.distance_to_vec3(&self.p3);
        let c = self.p3.distance_to_vec3(&self.p1);
        let s = (a + b + c) / T::from(2.0).expect("Failed to convert number to T");
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    pub fn bounds(&self) -> BoundingBox<T> {
        BoundingBox::new(
            Vec3::new(
                self.p1.x.min(self.p2.x).min(self.p3.x),
                self.p1.y.min(self.p2.y).min(self.p3.y),
                self.p1.z.min(self.p2.z).min(self.p3.z),
            ),
            Vec3::new(
                self.p1.x.max(self.p2.x).max(self.p3.x),
                self.p1.y.max(self.p2.y).max(self.p3.y),
                self.p1.z.max(self.p2.z).max(self.p3.z),
            ),
        )
    }

    pub fn closest_pt(&self, pt: &Vec3<T>) -> Vec3<T> {
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;
    
        // Compute vectors
        let ab = p2 - p1;
        let ac = p3 - p1;
        let ap = *pt - p1;
    
        // Compute barycentric coordinates
        let d1 = ab.dot(&ap);
        let d2 = ac.dot(&ap);
        if d1 <= T::zero() && d2 <= T::zero() {
            return p1; // Barycentric coordinates (1,0,0)
        }
    
        // Check if P in vertex region outside p2
        let bp = *pt - p2;
        let d3 = ab.dot(&bp);
        let d4 = ac.dot(&bp);
        if d3 >= T::zero() && d4 <= d3 {
            return p2; // Barycentric coordinates (0,1,0)
        }
    
        // Check if P in edge region of AB, if so return projection of P onto AB
        let vc = d1 * d4 - d3 * d2;
        if vc <= T::zero() && d1 >= T::zero() && d3 <= T::zero() {
            let v = d1 / (d1 - d3);
            return p1 + ab * v; // Barycentric coordinates (1-v,v,0)
        }
    
        // Check if P in vertex region outside p3
        let cp = *pt - p3;
        let d5 = ab.dot(&cp);
        let d6 = ac.dot(&cp);
        if d6 >= T::zero() && d5 <= d6 {
            return p3; // Barycentric coordinates (0,0,1)
        }
    
        // Check if P in edge region of AC, if so return projection of P onto AC
        let vb = d5 * d2 - d1 * d6;
        if vb <= T::zero() && d2 >= T::zero() && d6 <= T::zero() {
            let w = d2 / (d2 - d6);
            return p1 + ac * w; // Barycentric coordinates (1-w,0,w)
        }
    
        // Check if P in edge region of BC, if so return projection of P onto BC
        let va = d3 * d6 - d5 * d4;
        if va <= T::zero() && (d4 - d3) >= T::zero() && (d5 - d6) >= T::zero() {
            let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
            return p2 + (p3 - p2) * w; // Barycentric coordinates (0,1-w,w)
        }
    
        // P inside face region. Compute Q through its barycentric coordinates (u,v,w)
        let denom = T::one() / (va + vb + vc);
        let v = vb * denom;
        let w = vc * denom;
        p1 + ab * v + ac * w // Barycentric coordinates (1-v-w,v,w)
    }
    
}
