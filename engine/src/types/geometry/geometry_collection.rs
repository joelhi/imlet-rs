use num_traits::Float;

use crate::types::computation::traits::ImplicitFunction;

use super::{
    traits::{SignedDistance, SignedQuery, SpatialQuery},
    BoundingBox, Mesh, Octree, Triangle, Vec3,
};

/// Stores a fixed, queryable collection of primitive geometries.
///
/// Internally stores the objects inside an octree.
pub struct GeometryCollection<Q, T> {
    tree: Octree<Q, T>,
}

impl<Q: SpatialQuery<T>, T: Float> GeometryCollection<Q, T> {
    /// Build a collection from a set of objects.
    ///
    /// # Arguments
    ///
    /// * `objects` - List of objects to store in collection.
    pub fn build(objects: Vec<Q>) -> Self {
        let mut tree = Octree::new(10, 12);
        let bounds = BoundingBox::from_objects(&objects);
        tree.build(bounds, objects);

        Self { tree }
    }

    /// Find the closest point on any object in the collection.
    ///
    /// # Arguments
    ///
    /// * `query_point` - Point to find closest point to.
    pub fn closest_point(&self, query_point: &Vec3<T>) -> (Vec3<T>, Q) {
        self.tree.closest_point(query_point)
    }
}

impl<T: Float> GeometryCollection<Triangle<T>, T> {
    /// Generate a collection of triangles from a mesh
    ///
    /// # Arguments
    ///
    /// * `mesh` - Mesh to generate collection of triangles from.
    pub fn from_mesh(mesh: &Mesh<T>) -> GeometryCollection<Triangle<T>, T> {
        let triangles = mesh.as_triangles();

        GeometryCollection::build(triangles)
    }
}

impl<Q: SignedQuery<T>, T: Float> SignedDistance<T> for GeometryCollection<Q, T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.tree.signed_distance(&Vec3::new(x, y, z))
    }
}

impl<Q: SignedQuery<T> + Send + Sync, T: Float + Send + Sync> ImplicitFunction<T>
    for GeometryCollection<Q, T>
{
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }
}
