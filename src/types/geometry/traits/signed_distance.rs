/// Trait to expose a method to compute the signed distance.
///
/// This trait is used to allow object to be passed to an implicit model as a signed distance function.
pub trait SignedDistance<T> {
    /// Compute the signed distance from some coordinates.
    fn signed_distance(&self, x: T, y: T, z: T) -> T;
}
