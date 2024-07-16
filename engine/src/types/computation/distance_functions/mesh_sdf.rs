use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::ImplicitFunction,
    geometry::{Mesh, OctreeNode, Vec3},
};

#[derive(Debug)]
pub struct MeshSDF<T: Float + Debug> {
    pub tree: Box<OctreeNode<T>>,
}

impl<'a, T: Float + Debug + Send + Sync> MeshSDF<T> {
    pub fn new(mesh: &Mesh<T>, max_depth: u32, max_triangles: usize) -> Self {
        let tree = mesh.compute_octree(max_depth, max_triangles);
        Self {
            tree: Box::new(tree),
        }
    }
}

impl<'a, T: Float + Debug + Send + Sync> ImplicitFunction<T> for MeshSDF<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let query = Vec3::new(x, y, z);

        self.tree.signed_distance(query, true)
    }
}
