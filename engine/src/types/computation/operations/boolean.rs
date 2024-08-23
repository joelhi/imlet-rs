use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::implicit_functions::ImplicitOperation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Union {
}

impl Union {
    pub fn new() -> Self {
        Self { }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Union {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].min(inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Intersection {
}

impl Intersection {
    pub fn new() -> Self {
        Self { }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Intersection {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Difference {
}

impl Difference {
    pub fn new() -> Self {
        Self { }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Difference {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(-inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}
