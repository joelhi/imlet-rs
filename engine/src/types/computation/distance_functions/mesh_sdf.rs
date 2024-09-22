use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::traits::ImplicitFunction,
    geometry::{Mesh, Octree, Triangle, Vec3},
};

/// Distance function for an arbitrary triangle mesh.
///
/// This will create an octree of the mesh triangles and use that for signed distance queries.
#[derive(Debug)]
pub struct MeshSDF<T> {
    /// Octree of triangles to use for signed distance computation
    pub tree: Box<Octree<Triangle<T>, T>>,
}

impl<T: Float> MeshSDF<T> {
    /// Create a new Mesh SDF operation. This method will take in the mesh and build the octree.
    /// # Panics
    ///
    /// This method may panic if the octree construction fails.
    ///
    /// # Arguments
    ///
    /// * `max_depth` - Maximum allowed recursive depth when constructing the octree.
    /// * `max_triangles` - Maximum number of triangles per leaf node.
    pub fn build(mesh: &Mesh<T>, max_depth: u32, max_triangles: usize) -> Self {
        let tree = mesh.compute_octree(max_depth, max_triangles);
        Self {
            tree: Box::new(tree),
        }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for MeshSDF<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let query = Vec3::new(x, y, z);
        self.tree.signed_distance(&query)
    }
}
