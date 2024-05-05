use crate::engine::types::computation::component::ImplicitOperation;

pub struct Union {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Union {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].min(inputs[1])
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}

pub struct Intersection {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Intersection {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].max(inputs[1])
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}

pub struct Difference {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Difference {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0].max(-inputs[1])
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}