use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Vec3;

/// Single line segment defined by a start and end point.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Line<T> {
    pub start: Vec3<T>,
    pub end: Vec3<T>,
}

impl<T> Line<T> {
    /// Create a new Line from a start and end point.
    ///
    /// # Arguments
    ///
    /// * `start` - Start of the line.
    /// * `end` - End of the line.
    pub fn new(start: Vec3<T>, end: Vec3<T>) -> Self {
        Self { start, end }
    }
}

impl<T: Float> Line<T> {
    /// Computes the distance to the closest point on the line from a point.
    ///
    /// # Arguments
    ///
    /// * `point` - Point from which distance is computed.
    pub fn distance_to(&self, point: Vec3<T>) -> T {
        self.closest_pt(point).distance_to_vec3(&point)
    }

    /// Computes the closest point on the line from a point.
    ///
    /// # Arguments
    ///
    /// * `point` - Point from which the closest point is computed.
    pub fn closest_pt(&self, pt: Vec3<T>) -> Vec3<T> {
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(&v2)).clamp(T::zero(), self.start.distance_to_vec3(&self.end));
        self.start + (v2 * t)
    }

    /// Compute the length of the line.
    pub fn length(&self) -> T {
        self.start.distance_to_vec3(&self.end)
    }
}
