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
        BoundingBox { min, max }
    }

    pub fn get_dimensions(&self) -> (T, T, T) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn is_inside(&self, pt: Vec3<T>) -> bool {
        pt.x > self.min.x
            && pt.y > self.min.y
            && pt.z > self.min.z
            && pt.x < self.max.x
            && pt.y < self.max.y
            && pt.z < self.max.z
    }

    pub fn is_coord_inside(&self, x: T, y: T, z: T) -> bool {
        x > self.min.x
            && y > self.min.y
            && z > self.min.z
            && x < self.max.x
            && y < self.max.y
            && z < self.max.z
    }
}
