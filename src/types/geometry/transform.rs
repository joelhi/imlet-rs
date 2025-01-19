use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Vec3;

/// A Transform operation.
///
/// The operation is defined as a translation and rotation component.
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

    /// Create a new transform from a translation.
    pub fn translation(translation: Vec3<T>) -> Self {
        Transform::new(translation, Vec3::origin())
    }

    /// Create a new transform from a rotation.
    pub fn rotation(rotation: Vec3<T>) -> Self {
        Transform::new(Vec3::origin(), rotation)
    }
}
