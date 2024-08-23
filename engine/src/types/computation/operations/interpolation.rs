use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::implicit_functions::ImplicitOperation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LinearInterpolation {}

impl LinearInterpolation {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for LinearInterpolation {
    fn eval(&self, inputs: &[T]) -> T {
        let zero = T::zero();
        let one = T::one();
        let t = inputs[2].clamp(zero, one);
        inputs[0] + t * (inputs[1] - inputs[0])
    }

    fn num_inputs(&self) -> usize {
        3
    }
}
