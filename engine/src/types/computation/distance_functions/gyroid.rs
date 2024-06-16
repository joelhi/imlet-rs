use num_traits::Float;

use crate::{types::computation::component::ImplicitFunction, utils::math_helper::Pi};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Gyroid<T: Pi + Float + Debug> {
    pub length_x: T,
    pub length_y: T,
    pub length_z: T,
}

impl<T: Pi + Float + Debug> Gyroid<T> {
    pub fn new(length_x: T, length_y: T, length_z: T) -> Self {
        Self {
            length_x: length_x,
            length_y: length_y,
            length_z: length_z,
        }
    }

    pub fn with_equal_spacing(length: T) -> Self {
        Self {
            length_x: length,
            length_y: length,
            length_z: length,
        }
    }
}

impl<T: Pi + Float + Debug + Send + Sync> ImplicitFunction<T> for Gyroid<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let two = T::from(2.0).unwrap();
        (two * T::pi() * x / self.length_x).sin() * (two * T::pi() * y / self.length_y).cos()
            + (two * T::pi() * y / self.length_y).sin() * (two * T::pi() * z / self.length_z).cos()
            + (two * T::pi() * z / self.length_z).sin() * (two * T::pi() * x / self.length_x).cos()
    }
}
