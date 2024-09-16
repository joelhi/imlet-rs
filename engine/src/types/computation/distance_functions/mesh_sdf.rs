use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::traits::implicit_functions::ImplicitFunction,
    geometry::{Mesh, OctreeNode, Triangle, Vec3},
};

#[derive(Debug)]
pub struct MeshSDF<T: Float + Debug + Send + Sync> {
    pub tree: Box<OctreeNode<Triangle<T>, T>>,
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

        let source = Vec3::new(3.776, -1.434, 2.032);

        let dist = self.tree.signed_distance(&query);

        if query.to_f64().distance_to_vec3(&source) < 0.26 && dist.to_f64().unwrap() < 0.0 {
            println!("Point {}, Distance {}", query, dist.to_f64().unwrap());
        }

        dist
    }
}
