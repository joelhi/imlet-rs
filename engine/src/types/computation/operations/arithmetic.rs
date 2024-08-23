use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::implicit_functions::ImplicitOperation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Multiply {}

impl Multiply {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Multiply {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] * inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Add {}

impl Add {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Add {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Subtract {}

impl Subtract {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Subtract {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Divide {}

impl Divide {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Divide {
    fn eval(&self, inputs: &[T]) -> T {
        debug_assert!(inputs[1] != T::zero(), "Cannot divide by zero");
        inputs[0] / inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}
