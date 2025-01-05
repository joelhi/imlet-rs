use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Vec3;

/// A Transform operation.
///
/// The operation is defined as a translation and rotation component. Can be applied to object implementing the [`Transformable`](super::traits::Transformable) trait.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform<T> {
    pub translation: Vec3<T>,
    pub rotation: Vec3<T>,
}

impl<T: Float> Transform<T> {
    /// New transformation from a translation and a rotation.
    pub fn new(translation: Vec3<T>, rotation: Vec3<T>) -> Self {
        Transform {
            translation,
            rotation,
        }
    }

    pub fn translation(translation: Vec3<T>) -> Self {
        Transform::new(translation, Vec3::origin())
    }

    pub fn rotation(rotation: Vec3<T>) -> Self {
        Transform::new(Vec3::origin(), rotation)
    }
}
