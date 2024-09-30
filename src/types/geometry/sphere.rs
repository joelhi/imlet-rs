use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{computation::traits::ImplicitFunction, geometry::Vec3};

use super::{traits::SignedDistance, BoundingBox};

/// A sphere object, defined by a centre point and a radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere<T> {
    pub centre: Vec3<T>,
    pub radius: T,
}

impl<T> Sphere<T> {
    /// Create a new sphere.
    /// # Arguments
    ///
    /// * `centre` -The centre point of the sphere.
    /// * `radius` -The radius of the sphere
    pub fn new(centre: Vec3<T>, radius: T) -> Self {
        Self { centre, radius }
    }

    /// Create a new sphere at a point defined by x, y and z coordinates.
    /// # Arguments
    ///
    /// * `x` -The x coord of the centre of the sphere.
    /// * `y` -The y coord of the centre of the sphere.
    /// * `z` -The z coord of the centre of the sphere.
    /// * `radius` -The radius of the sphere
    pub fn at_coord(x: T, y: T, z: T, radius: T) -> Self {
        Self {
            centre: Vec3::new(x, y, z),
            radius,
        }
    }
}

impl<T: Float> Sphere<T> {
    /// Compute the bounding box for the sphere
    pub fn bounds(&self) -> BoundingBox<T> {
        let min = self.centre - Vec3::new(self.radius, self.radius, self.radius);
        let max = self.centre + Vec3::new(self.radius, self.radius, self.radius);
        BoundingBox::new(min, max)
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Sphere<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.centre.distance_to_coord(x, y, z) - self.radius
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Sphere<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.centre.distance_to_coord(x, y, z) - self.radius
    }
}
