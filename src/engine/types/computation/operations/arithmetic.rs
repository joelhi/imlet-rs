use crate::engine::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Multiply {
    inputs: [ComponentId; 2],
}

impl Multiply {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Multiply { inputs: [a, b] }
    }
}

impl ImplicitOperation for Multiply {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0] * inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Add {
    inputs: [ComponentId; 2],
}

impl Add {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Add { inputs: [a, b] }
    }
}

impl ImplicitOperation for Add {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0] + inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
pub struct Subtract {
    inputs: [ComponentId; 2],
}

impl Subtract {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Subtract { inputs: [a, b] }
    }
}

impl ImplicitOperation for Subtract {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0] - inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Divide {
    inputs: [ComponentId; 2],
}

impl Divide {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Divide { inputs: [a, b] }
    }
}

impl ImplicitOperation for Divide {
    fn eval(&self, inputs: &[f32]) -> f32 {
        assert!(inputs[1] > 0.0, "Cannot divide by zero");
        inputs[0] / inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
