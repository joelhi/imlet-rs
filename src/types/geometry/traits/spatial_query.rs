use crate::types::geometry::Vec3;

use super::Bounded;

/// Trait to expose behaviour used for closest point queries.
///
/// This trait is mainly used to allow a struct to be stored and processed as part of an octree.
pub trait SpatialQuery<T>: Copy + Bounded<T> {
    /// Returns the closest point on the object based on a query point.
    fn closest_point(&self, query_point: &Vec3<T>) -> Vec3<T>;

    /// Returns a default instance of the object.
    fn default() -> Self;
}

/// Extension to allow closest point look-ups to be classified as inside or outside, based on the normal.
///
/// This is mainly used for computing signed distances to objects stored in an octree.
pub trait SignedQuery<T>: SpatialQuery<T> {
    /// Returns the closest point and the normal at the closest point from a query point.
    ///
    /// # Arguments
    ///
    /// * `query_point` - Point to which the closest point is computed.
    ///
    /// # Returns
    ///
    /// * A tuple ([`Vec3`],[`Vec3`]) where the first element is the closest point and the second the normal at the point.
    fn closest_point_with_normal(&self, query_point: &Vec3<T>) -> (Vec3<T>, Vec3<T>);
}
