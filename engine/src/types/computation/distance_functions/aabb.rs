use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::traits::implicit_functions::ImplicitFunction,
    geometry::{BoundingBox, Vec3},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB<T: Float + Debug> {
    pub bounds: BoundingBox<T>,
}

impl<T: Float + Debug> AABB<T> {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            bounds: BoundingBox::new(min, max),
        }
    }

    pub fn from_size(origin: Vec3<T>, size: T) -> Self {
        Self {
            bounds: BoundingBox::new(
                origin,
                Vec3::new(origin.x + size, origin.y + size, origin.z + size),
            ),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for AABB<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let point = Vec3::new(x, y, z);

        self.bounds.signed_distance(&point)
    }
}
