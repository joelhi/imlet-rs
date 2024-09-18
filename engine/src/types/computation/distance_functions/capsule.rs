use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{computation::traits::implicit_functions::ImplicitFunction, geometry::Vec3};

/// Distance function for a capsule defined by a line and a radius
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Capsule<T: Float + Debug> {
    start: Vec3<T>,
    end: Vec3<T>,
    radius: T,
}

impl<T: Float + Debug> Capsule<T> {
    /// Creare a new Capsule.
    /// # Arguments
    /// 
    /// * `start` - Start of line for capsule length.
    /// * `end` - End of line for capsule length.
    /// * `radius` - Capsule radius.
    pub fn new(start: Vec3<T>, end: Vec3<T>, radius: T) -> Self {
        Capsule {
            start: start,
            end: end,
            radius: radius,
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Capsule<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let zero = T::zero();
        let pt = Vec3::new(x, y, z);
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(&v2)).clamp(zero, self.start.distance_to_vec3(&self.end));
        let pt_on_line = self.start + (v2 * t);
        pt_on_line.distance_to_vec3(&pt) - self.radius
    }
}
