use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Vec3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Line<T: Float + Debug> {
    pub start: Vec3<T>,
    pub end: Vec3<T>,
}

impl<T: Float + Debug> Line<T> {
    pub fn new(start: Vec3<T>, end: Vec3<T>) -> Self {
        Self { start, end }
    }

    pub fn distance_to(&self, pt: Vec3<T>) -> T {
        self.closest_pt(pt).distance_to_vec3(&pt)
    }

    pub fn closest_pt(&self, pt: Vec3<T>) -> Vec3<T> {
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(&v2)).clamp(T::zero(), self.start.distance_to_vec3(&self.end));
        self.start + (v2 * t)
    }

    pub fn length(&self) -> T {
        self.start.distance_to_vec3(&self.end)
    }
}
