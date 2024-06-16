use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::component::ImplicitFunction;
use crate::utils::math_helper::Pi;

#[derive(Debug, Clone, Copy)]
pub struct Neovius<T: Pi + Float + Debug> {
    pub length_x: T,
    pub length_y: T,
    pub length_z: T,
}

impl<T: Pi + Float + Debug> Neovius<T> {
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

impl<T: Pi + Float + Debug + Send + Sync> ImplicitFunction<T> for Neovius<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let two = T::from(2.0).expect("Failed to convert number to T");
        let three = T::from(2.0).expect("Failed to convert number to T");
        let four = T::from(4.0).expect("Failed to convert number to T");
        let x = two * T::pi() * x / self.length_x;
        let y = two * T::pi() * y / self.length_y;
        let z = two * T::pi() * z / self.length_z;
        three * (x.cos() + y.cos() + z.cos()) + four * x.cos() * y.cos() * z.cos()
    }
}
