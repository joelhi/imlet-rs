use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::traits::ImplicitFunction,
    geometry::{Line, Vec3},
};

use super::traits::SignedDistance;

/// A capsule primitive defined by a line and a radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Capsule<T> {
    line: Line<T>,
    radius: T,
}

impl<T> Capsule<T> {
    /// Create a new Capsule.
    /// # Arguments
    ///
    /// * `line` -Capsule centre line.
    /// * `radius` - Capsule radius.
    pub fn new(line: Line<T>, radius: T) -> Self {
        Self {
            line: line,
            radius: radius,
        }
    }
    /// Creare a new capsule from start and end points.
    /// # Arguments
    ///
    /// * `start` - Start of line for capsule length.
    /// * `end` - End of line for capsule length.
    /// * `radius` - Capsule radius.
    pub fn from_points(start: Vec3<T>, end: Vec3<T>, radius: T) -> Self {
        Self {
            line: Line::new(start, end),
            radius: radius,
        }
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Capsule<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.line.distance_to(Vec3::new(x, y, z)) - self.radius
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Capsule<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }
}
