use num_traits::Float;

use crate::types::computation::component::ImplicitFunction;
use crate::utils::math_helper::Pi;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct SchwarzP<T: Pi + Float + Debug> {
    pub length_x: T,
    pub length_y: T,
    pub length_z: T,
}

impl<T: Pi + Float + Debug> SchwarzP<T> {
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

impl<T: Float + Debug + Send + Sync + Pi> ImplicitFunction<T> for SchwarzP<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let two = T::from(2.0).expect("Failed to convert number to T");
        let x = (two * T::pi() * x / self.length_x) as T;
        let y = (two * T::pi() * y / self.length_y) as T;
        let z = (two * T::pi() * z / self.length_z) as T;
        x.cos() + y.cos() + z.cos()
    }
}
