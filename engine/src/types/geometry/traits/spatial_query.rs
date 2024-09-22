use crate::types::geometry::{BoundingBox, Vec3};

/// Trait to expose behaviour used for closest point queries.
///
/// This trait is mainly used to allow a struct to be stored and processed as part of an octree.
pub trait SpatialQuery<T>: Copy {
    fn bounds(&self) -> BoundingBox<T>;

    fn closest_point(&self, query_point: &Vec3<T>) -> Vec3<T>;

    fn default() -> Self;
}

/// Extension to allow closest point look-ups to be classified as inside or outside, based on the normal.
///
/// This is mainly used for computing signed distances to objects stored in an octree.
pub trait SignedQuery<T>: SpatialQuery<T> {
    fn normal_at(&self, query_point: &Vec3<T>) -> Vec3<T>;
}
