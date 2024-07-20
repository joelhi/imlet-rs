use std::fmt::Debug;

use num_traits::Float;

use crate::types::{computation::traits::implicit_functions::ImplicitFunction, geometry::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Sphere<T: Float + Debug> {
    pub source: Vec3<T>,
    pub radius: T,
}

impl<T: Float + Debug> Sphere<T> {
    pub fn new(source: Vec3<T>, radius: T) -> Self {
        Self { source, radius }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Sphere<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.source.distance_to_coord(x, y, z) - self.radius
    }
}
