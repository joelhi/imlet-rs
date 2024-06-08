use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::ImplicitFunction,
    geometry::{BoundingBox, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct AABB<T: Float + Debug> {
    pub bounds: BoundingBox<T>,
}

impl<T: Float + Debug> AABB<T> {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            bounds: BoundingBox::new(min, max),
        }
    }

    pub fn from_size(origin: Vec3<T>, size: T) -> Self {
        Self {
            bounds: BoundingBox::new(
                origin,
                Vec3::new(origin.x + size, origin.y + size, origin.z + size),
            ),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for AABB<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let pt = Vec3::new(x, y, z);

        let diff1 = self.bounds.max - pt;
        let diff2 = self.bounds.min - pt;

        let dist = diff1.x.abs().min(
            diff1.y.abs().min(
                diff1
                    .z
                    .abs()
                    .min(diff2.x.abs().min(diff2.y.abs().min(diff2.z.abs()))),
            ),
        );

        if self.bounds.is_coord_inside(x, y, z) {
            -dist
        } else {
            dist
        }
    }
}
