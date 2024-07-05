use std::fmt::Debug;

use num_traits::Float;

use super::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox<T: Float + Debug> {
    pub min: Vec3<T>,
    pub max: Vec3<T>,
}

impl<T: Float + Debug> BoundingBox<T> {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }

    pub fn ZERO()-> Self{
        Self {min: Vec3::origin(), max: Vec3::origin()}
    }

    pub fn dimensions(&self) -> (T, T, T) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn contains(&self, pt: Vec3<T>) -> bool {
        pt.x > self.min.x
            && pt.y > self.min.y
            && pt.z > self.min.z
            && pt.x < self.max.x
            && pt.y < self.max.y
            && pt.z < self.max.z
    }

    pub fn contains_coord(&self, x: T, y: T, z: T) -> bool {
        x > self.min.x
            && y > self.min.y
            && z > self.min.z
            && x < self.max.x
            && y < self.max.y
            && z < self.max.z
    }

    pub fn intersects(&self, other: &BoundingBox<T>) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}
