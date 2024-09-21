use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{computation::traits::ImplicitFunction, geometry::Vec3};

/// Distance function for a Sphere, defined by a centre point and a radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere<T: Float + Debug> {
    pub centre: Vec3<T>,
    pub radius: T,
}

impl<T: Float + Debug> Sphere<T> {
    /// Create a new sphere.
    /// # Arguments
    ///
    /// * `centre` -The centre point of the sphere.
    /// * `radius` -The radius of the sphere
    pub fn new(centre: Vec3<T>, radius: T) -> Self {
        Self { centre, radius }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Sphere<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.centre.distance_to_coord(x, y, z) - self.radius
    }
}
