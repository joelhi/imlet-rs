use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::implicit_functions::ImplicitFunction;
use crate::utils::math_helper::Pi;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Neovius<T: Pi + Float + Debug> {
    pub length_x: T,
    pub length_y: T,
    pub length_z: T,
    pub linear: bool,
}

impl<T: Pi + Float + Debug> Neovius<T> {
    pub fn new(length_x: T, length_y: T, length_z: T, linear: bool) -> Self {
        Self {
            length_x: length_x,
            length_y: length_y,
            length_z: length_z,
            linear: linear,
        }
    }

    pub fn with_equal_spacing(length: T, linear: bool) -> Self {
        Self {
            length_x: length,
            length_y: length,
            length_z: length,
            linear: linear,
        }
    }
}

impl<T: Pi + Float + Debug + Send + Sync> ImplicitFunction<T> for Neovius<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let two = T::from(2.0).expect("Failed to convert number to T");
        let three = T::from(2.0).expect("Failed to convert number to T");
        let four = T::from(4.0).expect("Failed to convert number to T");
        let x = two * T::pi() * x / self.length_x;
        let y = two * T::pi() * y / self.length_y;
        let z = two * T::pi() * z / self.length_z;
        let normalized_distance =
            three * (x.cos() + y.cos() + z.cos()) + four * x.cos() * y.cos() * z.cos();

        let scale = self.length_x.min(self.length_y).min(self.length_z) / two;

        if self.linear {
            let linear_distance =
                normalized_distance.clamp(-T::one(), T::one()).asin() / (T::pi() / two);
            scale * linear_distance
        } else {
            scale * normalized_distance
        }
    }
}
