use hashbrown::HashSet;
use std::fmt::Debug;
use std::time::Instant;

use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::geometry::{BoundingBox, Transform, Vec3};

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::types::computation::traits::ModelFloat;
use crate::utils;

use super::Line;
use super::Octree;
use super::SpatialHashGrid;
use super::Triangle;

/// A triangle mesh.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Mesh<T> {
    vertices: Vec<Vec3<T>>,
    faces: Vec<[usize; 3]>,
    normals: Option<Vec<Vec3<T>>>,
}

impl<T> Default for Mesh<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Mesh<T> {
    /// Create a new empty mesh
    pub fn new() -> Mesh<T> {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            normals: None,
        }
    }

    /// Returns the vertices of the mesh
    pub fn vertices(&self) -> &[Vec3<T>] {
        &self.vertices
    }

    /// Returns the faces of the mesh.
    pub fn faces(&self) -> &[[usize; 3]] {
        &self.faces
    }

    /// Returns the optional vertex normals of the mesh.
    pub fn normals(&self) -> Option<&[Vec3<T>]> {
        self.normals.as_deref()
    }

    /// Total number of vertices
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Total number of faces
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }
}

impl<T: Float> Mesh<T> {
    /// Add vertices to the vertex list of the mesh
    /// # Arguments
    ///
    /// * `vertices` - Slice of vertices to be added.
    pub fn add_vertices(&mut self, vertices: &[Vec3<T>]) {
        self.vertices.extend_from_slice(vertices);
    }

    /// Add faces to the face list of the mesh
    /// # Arguments
    ///
    /// * `faces` - Slice of faces to be added.
    pub fn add_faces(&mut self, faces: &[[usize; 3]]) {
        self.faces.extend_from_slice(faces);
    }

    /// Set explicit vertex normals for each vertex.
    ///
    /// Returns true if normals are sucessfully assigned.
    pub(crate) fn set_normals(&mut self, normals: &[Vec3<T>]) -> bool {
        if normals.len() != self.vertices.len() {
            false
        } else {
            self.normals = Some(normals.to_vec());
            true
        }
    }

    /// Returns the unique edges of the mesh.
    pub fn edges(&self) -> Vec<Line<T>> {
        let mut edges_i = HashSet::with_capacity(self.num_faces());
        for f in self.faces.iter() {
            let mut sorted = *f;
            sorted.sort();
            edges_i.insert((f[0], f[1]));
            edges_i.insert((f[1], f[2]));
            edges_i.insert((f[2], f[0]));
        }

        edges_i
            .iter()
            .map(|(i, j)| Line::new(self.vertices[*i], self.vertices[*j]))
            .collect()
    }

    /// Computes the average of all mesh vertices.
    pub fn centroid(&self) -> Vec3<T> {
        let mut centroid: Vec3<T> = Vec3::origin();

        for &v in self.vertices() {
            centroid = centroid + v;
        }

        centroid * T::from(1.0 / self.num_vertices() as f64).expect("Failed to convert number to T")
    }

    /// Bounding box of mesh in global coordinates.
    pub fn bounds(&self) -> BoundingBox<T> {
        let mut max = Vec3::new(-T::max_value(), -T::max_value(), -T::max_value());
        let mut min = Vec3::new(T::max_value(), T::max_value(), T::max_value());

        for v in self.vertices() {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);

            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        BoundingBox::new(min, max)
    }

    /// Computes and stores the vertex normals using an angle weighted average of the incident faces.
    pub fn compute_vertex_normals(&mut self) {
        let before = Instant::now();

        let face_normals: Vec<Vec3<T>> = self.compute_face_normals();
        let vertex_faces: Vec<Vec<usize>> = self.compute_vertex_faces();
        let mut vertex_normals = vec![Vec3::origin(); self.num_vertices()];
        vertex_normals.iter_mut().enumerate().for_each(|(id, n)| {
            for &f in &vertex_faces[id] {
                *n = *n + (face_normals[f] * self.face_angle_at_vertex(id, f));
            }
            *n = n.normalize();
        });
        self.normals = Some(vertex_normals);

        log::info!(
            "Mesh normals computed for {} points in {:.2?}",
            utils::math_helper::format_integer(self.num_vertices()),
            before.elapsed()
        );
    }

    pub(crate) fn compute_face_normals(&self) -> Vec<Vec3<T>> {
        self.faces
            .iter()
            .map(|f| {
                let v1 = self.vertices[f[1]] - self.vertices[f[0]];
                let v2 = self.vertices[f[2]] - self.vertices[f[0]];
                v1.cross(&v2).normalize()
            })
            .collect()
    }

    pub(crate) fn compute_vertex_faces(&self) -> Vec<Vec<usize>> {
        let mut vertex_faces = vec![Vec::with_capacity(12); self.num_vertices()];
        self.faces.iter().enumerate().for_each(|(id, f)| {
            vertex_faces[f[0]].push(id);
            vertex_faces[f[1]].push(id);
            vertex_faces[f[2]].push(id);
        });
        vertex_faces
    }

    fn face_angle_at_vertex(&self, vertex_index: usize, face_index: usize) -> T {
        let face = self.faces[face_index];
        let vertex = self.vertices[vertex_index];
        if face[0] == vertex_index {
            let v1 = self.vertices[face[1]] - vertex;
            let v2 = self.vertices[face[2]] - vertex;

            v1.angle(&v2).unwrap_or(T::zero())
        } else if face[1] == vertex_index {
            let v1 = self.vertices[face[0]] - vertex;
            let v2 = self.vertices[face[2]] - vertex;

            return v1.angle(&v2).unwrap_or(T::zero());
        } else if face[2] == vertex_index {
            let v1 = self.vertices[face[0]] - vertex;
            let v2 = self.vertices[face[1]] - vertex;

            return v1.angle(&v2).unwrap_or(T::zero());
        } else {
            panic!("Vertex not found in adjacent face. Mesh topology must be corrupt.")
        }
    }

    /// Convert the vertex data type from the current type to a new type Q.
    pub fn convert<Q: Float>(&self) -> Option<Mesh<Q>> {
        let converted_v: Vec<Vec3<Q>> = self
            .vertices
            .iter()
            .filter_map(|v| v.convert::<Q>())
            .collect();

        if converted_v.len() != self.vertices.len() {
            // Conversion failed for some vertices.
            return None;
        }

        let mut m = Mesh::<Q>::new();

        m.add_vertices(&converted_v);
        m.add_faces(&self.faces);

        if self.normals.is_some() {
            m.compute_vertex_normals();
        }

        Some(m)
    }

    /// Convert the mesh into a list of triangles. The triangles will store the mesh vertex normals if present.
    pub fn as_triangles(&self) -> Vec<Triangle<T>> {
        let mut triangles: Vec<Triangle<T>> = Vec::with_capacity(self.num_faces());
        for face in self.faces.iter() {
            let face_normals = self
                .normals
                .as_ref()
                .map(|n| [n[face[0]], n[face[1]], n[face[2]]]);
            triangles.push(Triangle::with_normals(
                self.vertices[face[0]],
                self.vertices[face[1]],
                self.vertices[face[2]],
                face_normals,
            ));
        }
        triangles
    }

    /// Compute an octree from the triangles of the mesh. Used for closest point and signed distance queries.
    /// # Arguments
    ///
    /// * `max_depth` - Maximum allowed recursive depth when constructing the tree.
    /// * `max_triangles` - Maximum number of triangles per leaf node.
    pub fn compute_octree(&self, max_depth: usize, max_triangles: usize) -> Octree<Triangle<T>, T> {
        let before = Instant::now();
        let tree = Octree::new()
            .with_max_depth(max_depth)
            .with_max_leaf_size(max_triangles)
            .with_objects(&self.as_triangles())
            .build();

        log::info!(
            "Octree computed for mesh with {} triangles in {:.2?}",
            utils::math_helper::format_integer(self.num_faces()),
            before.elapsed()
        );

        tree
    }
}

impl<T: ModelFloat> Mesh<T> {
    /// Create a indexed mesh from a list of triangle objects.
    ///
    /// # Arguments
    /// * `triangles` - slice of triangles to create mesh from.
    /// * `compute_normals` - If true will compute the smooth mesh normals for the vertices.
    /// * `tolerance` - Optional tolerance for merging vertices. If [`None`] will use default provided by [`Vec3::default_tolerance`]
    pub fn from_triangles(
        triangles: &[Triangle<T>],
        compute_normals: bool,
        tolerance: Option<T>,
    ) -> Mesh<T> {
        let tol = tolerance.unwrap_or(Vec3::default_tolerance());
        let before = Instant::now();
        let mut faces: Vec<[usize; 3]> = Vec::with_capacity(triangles.len());
        let mut grid = SpatialHashGrid::with_tolerance(tol);

        let mut mesh = Mesh::new();
        for triangle in triangles {
            let vertex_ids = [
                grid.add_point(triangle.p1()),
                grid.add_point(triangle.p2()),
                grid.add_point(triangle.p3()),
            ];

            if !(vertex_ids[0] == vertex_ids[1]
                || vertex_ids[0] == vertex_ids[2]
                || vertex_ids[1] == vertex_ids[2])
            {
                faces.push(vertex_ids);
            }
        }

        mesh.add_vertices(grid.vertices());
        mesh.add_faces(&faces);

        log::info!(
            "Mesh topology generated for {} points and {} triangles in {:.2?}",
            utils::math_helper::format_integer(mesh.num_vertices()),
            utils::math_helper::format_integer(mesh.num_faces()),
            before.elapsed()
        );

        if compute_normals {
            mesh.compute_vertex_normals_par();
        }

        mesh
    }

    /// Computes and stores the vertex normals using an angle weighted average of the incident faces using a parallel iterator.
    pub fn compute_vertex_normals_par(&mut self) {
        let before = Instant::now();

        let face_normals: Vec<Vec3<T>> = self.compute_face_normals_par();
        let vertex_faces: Vec<Vec<usize>> = self.compute_vertex_faces();
        let mut vertex_normals = vec![Vec3::origin(); self.num_vertices()];
        vertex_normals
            .par_iter_mut()
            .enumerate()
            .for_each(|(id, n)| {
                for &f in &vertex_faces[id] {
                    *n = *n + (face_normals[f] * self.face_angle_at_vertex(id, f));
                }
                *n = n.normalize();
            });
        self.normals = Some(vertex_normals);

        log::info!(
            "Mesh normals computed for {} points in {:.2?}",
            utils::math_helper::format_integer(self.num_vertices()),
            before.elapsed()
        );
    }

    pub(crate) fn compute_face_normals_par(&self) -> Vec<Vec3<T>> {
        self.faces
            .par_iter()
            .map(|f| {
                let v1 = self.vertices[f[1]] - self.vertices[f[0]];
                let v2 = self.vertices[f[2]] - self.vertices[f[0]];
                v1.cross(&v2).normalize()
            })
            .collect()
    }

    /// Apply a transformation to the mesh. This modifies the current mesh in-place.
    pub fn transform_self_par(&mut self, transform: Transform<T>) {
        self.vertices.par_iter_mut().for_each(|pt| {
            *pt = pt.transform(transform);
        });
    }
}
