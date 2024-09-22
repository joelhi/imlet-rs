use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::traits::ImplicitFunction,
    geometry::{BoundingBox, Vec3},
};

/// Distance function for an Axis Aligned Bounding Box (AABB)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AABB<T> {
    /// Box geometry
    pub bounds: BoundingBox<T>,
}

impl<T> AABB<T> {
    /// Creare a new AABB from extents.
    /// # Arguments
    ///
    /// * `min` - Minimum coordinate.
    /// * `max` - Maximum coordinate.
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            bounds: BoundingBox::new(min, max),
        }
    }

    /// Creare a new AABB from a BoundigBox.
    /// # Arguments
    ///
    /// * `box` - Box for distance function.
    pub fn from_bounds(bounds: BoundingBox<T>) -> Self {
        Self { bounds: bounds }
    }
}

impl<T: Float + Debug> AABB<T> {
    /// Creare a new AABB from a base point and a size.
    /// # Arguments
    ///
    /// * `origin` - Base point of the box.
    /// * `size` - Size of the box.
    pub fn from_size(origin: Vec3<T>, size: T) -> Self {
        Self {
            bounds: BoundingBox::new(
                origin,
                Vec3::new(origin.x + size, origin.y + size, origin.z + size),
            ),
        }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for AABB<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let point = Vec3::new(x, y, z);

        self.bounds.signed_distance(&point)
    }
}
