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
        let normalized_distance = (three * (x.cos() + y.cos() + z.cos())
            + four * x.cos() * y.cos() * z.cos())
            / T::from(10.0).unwrap();

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

#[cfg(test)]
mod tests {

    use core::f64;
    use num_traits::ToPrimitive;

    use super::*;

    #[test]
    fn test_compute_linearized() {
        let length = 5.0;
        let n = 10;
        let linear_neovius: Neovius<f64> = Neovius::with_equal_spacing(5.0, true);
        let mut max = 0.0;
        let mut min = f64::MAX;
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                for k in 0..n + 1 {
                    let x = length * i.to_f64().unwrap() / n.to_f64().unwrap();
                    let y = length * j.to_f64().unwrap() / n.to_f64().unwrap();
                    let z = length * k.to_f64().unwrap() / n.to_f64().unwrap();
                    max = max.max(linear_neovius.eval(x, y, z));
                    min = min.min(linear_neovius.eval(x, y, z));
                }
            }
        }
        assert!((2.5 - max).abs() < 0.001);
        assert!((min + 2.5).abs() < 0.001);
    }
}
