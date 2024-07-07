use std::fmt::Debug;

use num_traits::Float;

use super::{Line, Vec3};

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

    pub fn contains(&self, pt: &Vec3<T>) -> bool {
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

    pub fn corners(&self) -> [Vec3<T>; 8] {
        let delta = self.max - self.min;
        [
            self.min,
            self.min + Vec3::new(delta.x, T::zero(), T::zero()),
            self.min + Vec3::new(delta.x, delta.y, T::zero()),
            self.min + Vec3::new(T::zero(), delta.y, T::zero()),
            self.min + Vec3::new(T::zero(), T::zero(), delta.z),
            self.min + Vec3::new(delta.x, T::zero(), delta.z),
            self.max,
            self.min + Vec3::new(T::zero(), delta.y, delta.z),
        ]
    }

    pub fn wireframe(&self) -> [Line<T>; 12] {
        let corners = self.corners();
        [
            Line::new(corners[0], corners[1]),
            Line::new(corners[1], corners[2]),
            Line::new(corners[2], corners[3]),
            Line::new(corners[3], corners[0]),
            Line::new(corners[4], corners[5]),
            Line::new(corners[5], corners[6]),
            Line::new(corners[6], corners[7]),
            Line::new(corners[7], corners[4]),
            Line::new(corners[0], corners[4]),
            Line::new(corners[1], corners[5]),
            Line::new(corners[2], corners[6]),
            Line::new(corners[3], corners[7]),
        ]
    }

    pub fn centroid(&self) -> Vec3<T> {
        return (self.max + self.min) * T::from(0.5).expect("Failed to convert number to T");
    }

    pub fn intersects(&self, other: &BoundingBox<T>) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_corners() {
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(1.0, 1.0, 1.0));

        let corners = bounds.corners();

        assert!(corners[0].distance_to_coord(0.0, 0.0, 0.0) < 0.001);
        assert!(corners[1].distance_to_coord(1.0, 0.0, 0.0) < 0.001);
        assert!(corners[2].distance_to_coord(1.0, 1.0, 0.0) < 0.001);
        assert!(corners[3].distance_to_coord(0.0, 1.0, 0.0) < 0.001);
        assert!(corners[4].distance_to_coord(0.0, 0.0, 1.0) < 0.001);
        assert!(corners[5].distance_to_coord(1.0, 0.0, 1.0) < 0.001);
        assert!(corners[6].distance_to_coord(1.0, 1.0, 1.0) < 0.001);
        assert!(corners[7].distance_to_coord(0.0, 1.0, 1.0) < 0.001);
    }

    #[test]
    fn test_compute_corners_non_origin() {
        let bounds = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));

        let corners = bounds.corners();

        assert!(corners[0].distance_to_coord(1.0, 1.0, 1.0) < 0.001);
        assert!(corners[1].distance_to_coord(2.0, 1.0, 1.0) < 0.001);
        assert!(corners[2].distance_to_coord(2.0, 2.0, 1.0) < 0.001);
        assert!(corners[3].distance_to_coord(1.0, 2.0, 1.0) < 0.001);
        assert!(corners[4].distance_to_coord(1.0, 1.0, 2.0) < 0.001);
        assert!(corners[5].distance_to_coord(2.0, 1.0, 2.0) < 0.001);
        assert!(corners[6].distance_to_coord(2.0, 2.0, 2.0) < 0.001);
        assert!(corners[7].distance_to_coord(1.0, 2.0, 2.0) < 0.001);
    }

    #[test]
    fn test_compute_wireframe() {
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(1.0, 1.0, 1.0));

        let wireframe = bounds.wireframe();

        for line in wireframe {
            assert!(line.length() - 1.0 < 0.001);
        }
    }

    #[test]
    fn test_compute_wireframe_non_origin() {
        let bounds = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));

        let wireframe = bounds.wireframe();

        for line in wireframe {
            assert!(line.length() - 1.0 < 0.001);
        }
    }
}
