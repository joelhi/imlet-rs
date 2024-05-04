use crate::engine::types::functions::ImplicitFunction;

pub struct Node<'a> {
    function: &'a dyn ImplicitFunction,
    inputs: Vec<usize>,
}

impl<'a> Node<'a> {
    pub fn new(function: &'a dyn ImplicitFunction, inputs: Vec<usize>) -> Self {
        Node { function, inputs }
    }
    pub fn evaluate(inputs: Vec<f32>, x: f32, y: f32, z: f32) -> f32 {
        0.0
    }
}
