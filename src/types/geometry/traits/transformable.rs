use crate::types::geometry::Transform;

/// Trait to define a transformable object.
pub trait Transformable<T> {
    /// Trait to define a transformable object.
    /// 
    /// # Arguments
    /// 
    /// * `transform` - The transform to apply to the object.
    /// 
    /// Returns the transformed object.
    fn transform(&self, transform: Transform<T>)->Self;
}
