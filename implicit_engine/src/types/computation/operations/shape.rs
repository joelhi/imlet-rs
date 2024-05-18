use crate::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Offset {
    inputs: [ComponentId; 1],
    distance: f32
}

impl Offset{
    pub fn new(value: ComponentId, offset_distance: f32)->Self{
        Offset{
            inputs: [value],
            distance: offset_distance,
        }
    }
}

impl ImplicitOperation for Offset {
    fn eval(&self, inputs: &[f32]) -> f32 {
        inputs[0] - self.distance
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Thickness {
    inputs: [ComponentId; 1],
    t: f32
}

impl Thickness{
    pub fn new(value: ComponentId, thickness: f32)->Self{
        Thickness{
            inputs: [value],
            t: thickness,
        }
    }
}

impl ImplicitOperation for Thickness {
    fn eval(&self, inputs: &[f32]) -> f32 {
        (inputs[0] - self.t).max(-inputs[0])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}