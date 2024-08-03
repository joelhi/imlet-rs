use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{computation::traits::implicit_functions::ImplicitFunction, geometry::Plane};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
