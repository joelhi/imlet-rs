use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::ImplicitFunction,
    geometry::{OctreeNode, Vec3},
};

#[derive(Debug)]
pub struct MeshSDF<T: Float + Debug> {
    pub tree: Box<OctreeNode<T>>,
}

impl<'a, T: Float + Debug> MeshSDF<T> {
    pub fn new(tree: OctreeNode<T>) -> Self {
        Self {
            tree: Box::new(tree)
        }
    }
}

impl<'a, T: Float + Debug + Send + Sync> ImplicitFunction<T> for MeshSDF<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let query = Vec3::new(x, y, z);
        
        self.tree.signed_distance(query)
    }
}
