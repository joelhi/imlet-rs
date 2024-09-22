use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{computation::traits::ImplicitFunction, geometry::Plane};

/// Distance function for a plane, with a positive and negative side.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClippingPlane<T> {
    pub plane: Plane<T>,
}

impl<T: Float> ClippingPlane<T> {
    /// Create a new ClippingPlane from a Plane struct.
    pub fn new(plane: Plane<T>) -> Self {
        Self { plane: plane }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for ClippingPlane<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.plane.signed_distance_coord(x, y, z)
    }
}
