use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::{ComponentId, ImplicitOperation};

pub struct LinearInterpolation{
    inputs: [ComponentId; 3]
}

impl LinearInterpolation{
    pub fn new(value_a: ComponentId, value_b: ComponentId, t: ComponentId)->Self{
        Self{
            inputs: [value_a, value_b, t],
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for LinearInterpolation {
    fn eval(&self, inputs: &[T]) -> T {
        let zero = T::from(0.0).expect("Failed to convert number to T");
        let one = T::from(1.0).expect("Failed to convert number to T");
        let t = inputs[2].clamp(zero, one);
        inputs[0] + t * (inputs[1] - inputs[0])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}