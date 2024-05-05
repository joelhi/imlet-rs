use crate::engine::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Union {
    inputs: Vec<ComponentId>
}

impl Union{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Union{
            inputs: vec![a, b]
        }
    }
}

impl ImplicitOperation for Union {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].min(inputs[1])
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}

pub struct Intersection {
    inputs: Vec<ComponentId>
}

impl Intersection{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Intersection{
            inputs: vec![a, b]
        }
    }
}

impl ImplicitOperation for Intersection {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].max(inputs[1])
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}

pub struct Difference {
    inputs: Vec<ComponentId>
}

impl Difference{
    pub fn new(a: ComponentId, b: ComponentId)->Self{
        Difference{
            inputs: vec![a, b]
        }
    }
}

impl ImplicitOperation for Difference {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].max(-inputs[1])
    }

    fn get_inputs(&self) -> &Vec<ComponentId> {
        &self.inputs
    }
}