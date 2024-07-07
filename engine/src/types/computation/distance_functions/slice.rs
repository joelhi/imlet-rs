use std::fmt::Debug;

use num_traits::Float;

use crate::types::{computation::component::ImplicitFunction, geometry::Plane};

#[derive(Debug, Clone, Copy)]
pub struct Slice<T: Float + Debug> {
    pub plane: Plane<T>,
}

impl<T: Float + Debug> Slice<T> {
    pub fn new(plane: Plane<T>) -> Self {
        Self { plane: plane }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Slice<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.plane.signed_distance_coord(x, y, z)
    }
}
