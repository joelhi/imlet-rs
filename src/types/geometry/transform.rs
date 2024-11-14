use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Vec3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform<T> {
    pub translation: Vec3<T>,
    pub rotation: Vec3<T>,
}

impl<T: Float> Transform<T> {
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
