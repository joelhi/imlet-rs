use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::traits::implicit_functions::ImplicitFunction,
    geometry::{Mesh, OctreeNode, Triangle, Vec3},
};

#[derive(Debug)]
pub struct MeshSDF<T: Float + Debug + Send + Sync> {
    pub tree: Box<OctreeNode<Triangle<T>, T>>,
    proximity_tolerance: T,
}

impl<'a, T: Float + Debug + Send + Sync> MeshSDF<T> {
    pub fn new(mesh: &Mesh<T>, max_depth: u32, max_triangles: usize, proximity_tolerance: T) -> Self {
        let tree = mesh.compute_octree(max_depth, max_triangles);
        Self {
            tree: Box::new(tree),
            proximity_tolerance
        }
    }
}

impl<'a, T: Float + Debug + Send + Sync> ImplicitFunction<T> for MeshSDF<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let query = Vec3::new(x, y, z);

        self.tree.signed_distance(query, self.proximity_tolerance)
    }
}
