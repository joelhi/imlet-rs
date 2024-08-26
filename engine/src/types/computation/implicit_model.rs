use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use num_traits::Float;
use crate::types::computation::component::Component;
use crate::types::computation::ComponentId;
use crate::types::computation::traits::implicit_functions::{ImplicitFunction, ImplicitOperation};

pub struct ImplicitModel<T: Float + Debug>{
    components: HashMap<String, Component<T>>,
    inputs: HashMap<String, Vec<Option<String>>>
}

impl<T: Float + Debug + Send + Sync> ImplicitModel<T> {
    pub fn add_function<F: ImplicitFunction<T> + 'static>(&mut self, tag: String, function: F) {
        self.components.insert(tag.clone(), Component::Function(Box::new(function)));
        self.inputs.insert(tag.clone(), vec![]);
    }

    pub fn add_operation<F: ImplicitOperation<T> + 'static>(&mut self, tag: String, operation: F) {
        self.components.insert(tag.clone(), Component::Operation(Box::new(operation)));
        self.inputs.insert(tag.clone(), vec![None; operation.num_inputs()]);
    }

    pub fn add_constant(&mut self, tag: String, value: T) {
        self.components.insert(tag.clone(), Component::Constant(value));
        self.inputs.insert(tag.clone(), vec![]);
    }

    pub fn add_input(&mut self, target: &String, source: &String, index: usize) {
        let mut target_component_inputs = self.inputs.get(source).expect("Target component not found in model.");
        assert!(index < target_component_inputs.len(), "Input index out of bounds for target component. ");
        assert!(self.components.contains_key(source), "Source component not found in model. ");

        target_component_inputs[index] = Some(source.clone());
    }

    pub fn remove_input(&mut self, component: &String, index: usize) {
        let mut component_inputs = self.inputs.get(component).expect("Target component not found in model.");
        assert!(index < component_inputs.len(), "Input index out of bounds for target component. ");

        component_inputs[index] = None;
    }

    pub fn compile()->
}