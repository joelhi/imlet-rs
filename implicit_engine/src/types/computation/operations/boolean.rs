use crate::engine::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Union {
    inputs: [ComponentId; 2]
}

impl Union{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Union{
            inputs: [a, b]
        }
    }
}

impl ImplicitOperation for Union {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0].min(inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Intersection {
    inputs: [ComponentId; 2]
}

impl Intersection{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Intersection{
            inputs: [a, b]
        }
    }
}

impl ImplicitOperation for Intersection {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0].max(inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Difference {
    inputs: [ComponentId; 2]
}

impl Difference{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Difference{
            inputs: [a, b]
        }
    }
}

impl ImplicitOperation for Difference {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0].max(-inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}