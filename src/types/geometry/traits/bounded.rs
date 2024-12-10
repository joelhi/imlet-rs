use crate::types::geometry::BoundingBox;

/// Trait to define a bounded object.
pub trait Bounded<T> {
    /// Get the [`BoundingBox`] containing the object.
    fn bounds(&self) -> BoundingBox<T>;
}
