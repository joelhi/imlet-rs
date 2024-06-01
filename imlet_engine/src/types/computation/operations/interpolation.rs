use crate::types::computation::{ComponentId, ImplicitOperation};

pub struct LinearInterpolation {
    inputs: [ComponentId; 3]
}

impl LinearInterpolation{
    pub fn new(value_a: ComponentId, value_b: ComponentId, t: ComponentId)->Self{
        LinearInterpolation{
            inputs: [value_a, value_b, t],
        }
    }
}

impl ImplicitOperation for LinearInterpolation {
    fn eval(&self, inputs: &[f32]) -> f32 {
        let t = inputs[2].clamp(0.0, 1.0);
        inputs[0] + t * (inputs[1] - inputs[0])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}