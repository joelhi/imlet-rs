use crate::engine::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Multiply {
    inputs: Vec<ComponentId>
}

impl Multiply{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Multiply{
            inputs: vec![a, b],
        }
    }
}

impl ImplicitOperation for Multiply {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] * inputs[1]
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}

pub struct Add {
    inputs: Vec<ComponentId>
}

impl Add{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Add{
            inputs: vec![a, b],
        }
    }
}

impl ImplicitOperation for Add {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] + inputs[1]
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}
pub struct Subtract {
    inputs: Vec<ComponentId>
}

impl Subtract{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Subtract{
            inputs: vec![a, b],
        }
    }
}

impl ImplicitOperation for Subtract {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] - inputs[1]
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}

pub struct Divide {
    inputs: Vec<ComponentId>
}

impl Divide{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Divide{
            inputs: vec![a, b],
        }
    }
}

impl ImplicitOperation for Divide {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        assert!(inputs[1] > 0.0, "Cannot divide by zeros");
        inputs[0] / inputs[1]
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}
