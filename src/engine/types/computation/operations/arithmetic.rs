use crate::engine::types::computation::component::ImplicitOperation;

pub struct Multiply {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Multiply {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] * inputs[1]
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}

pub struct Add {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Add {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] + inputs[1]
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}
pub struct Subtract {
    a: usize,
    b: usize,
}

impl Subtract{
    pub fn new(a: usize, b: usize)->Self{
        Subtract{
            a,
            b
        }
    }
}

impl ImplicitOperation for Subtract {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        inputs[0] - inputs[1]
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}

pub struct Divide {
    a: usize,
    b: usize,
}

impl ImplicitOperation for Divide {
    fn eval(&self, inputs: &Vec<f32>) -> f32 {
        assert!(inputs[1] > 0.0, "Cannot divide by zero.");
        inputs[0] / inputs[1]
    }

    fn get_inputs(&self) -> Vec<&usize> {
        vec![&self.a, &self.b]
    }
}
