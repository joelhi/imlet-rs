use std::fmt::Debug;

use num_traits::Float;

use crate::types::{computation::component::ImplicitFunction, geometry::Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Torus<T: Float + Debug> {
    pub center: Vec3<T>,
    pub r: T,
    pub t: T,
}

impl<T: Float + Debug> Torus<T> {
    pub fn new(center: Vec3<T>, r: T, t: T) -> Self {
        Torus { center, r, t }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Torus<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        (self.r - ((x - self.center.x).powi(2) + (z - self.center.z).powi(2)).sqrt()).powi(2)
            + (y - self.center.y).powi(2)
            - self.t.powi(2)
    }
}
