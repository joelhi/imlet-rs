use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{traits::implicit_functions::ImplicitOperation, ComponentId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LinearInterpolation {
    inputs: [ComponentId; 3],
}

impl LinearInterpolation {
    pub fn new(value_a: ComponentId, value_b: ComponentId, t: ComponentId) -> Self {
        Self {
            inputs: [value_a, value_b, t],
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for LinearInterpolation {
    fn eval(&self, inputs: &[T]) -> T {
        let zero = T::zero();
        let one = T::one();
        let t = inputs[2].clamp(zero, one);
        inputs[0] + t * (inputs[1] - inputs[0])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
