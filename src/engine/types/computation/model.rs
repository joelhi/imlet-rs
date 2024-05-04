use crate::engine::types::functions::ImplicitFunction;

use super::node::Node;

pub struct Model<'a> {
    functions: Vec<Node<'a>>,
    outputs: Vec<Option<f32>>,
}

impl<'a> Model<'a> {
    pub fn add_function(
        &mut self,
        function: &'a dyn ImplicitFunction,
        inputs: Vec<usize>,
    ) -> usize {
        self.verify_inputs(function, &inputs);
        self.functions.push(Node::new(function, inputs));
        0
    }

    fn verify_inputs(&self, function: &'a dyn ImplicitFunction, inputs: &Vec<usize>) {
        assert!(
            function.num_params() == inputs.len(),
            "Incorrect number of inputs for function"
        );
        for index in inputs {
            assert!(
                *index < self.functions.len(),
                "Node index {index} not found in model."
            );
        }
    }

    pub fn evaluate(&mut self, x: f32, y: f32, z: f32) {
        // Build connectivity graph

        // Traverse graph and compute outputs
    }

    pub fn get_value(&self, node: usize) -> Option<f32> {
        self.outputs[node]
    }   
}
