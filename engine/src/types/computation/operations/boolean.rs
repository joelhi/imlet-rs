use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    component::ComponentId, traits::implicit_functions::ImplicitOperation,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Union {
    inputs: [ComponentId; 2],
}

impl Union {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Union {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].min(inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Intersection {
    inputs: [ComponentId; 2],
}

impl Intersection {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Intersection {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Difference {
    inputs: [ComponentId; 2],
}

impl Difference {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Difference {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(-inputs[1])
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
