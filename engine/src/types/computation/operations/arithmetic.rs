use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Multiply {
    inputs: [ComponentId; 2],
}

impl Multiply {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Multiply {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] * inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Add  {
    inputs: [ComponentId; 2],
}

impl Add {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Add {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
pub struct Subtract {
    inputs: [ComponentId; 2],
}

impl Subtract {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Subtract {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Divide {
    inputs: [ComponentId; 2],
}

impl Divide {
    pub fn new(a: ComponentId, b: ComponentId) -> Self {
        Self { inputs: [a, b] }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Divide {
    fn eval(&self, inputs: &[T]) -> T {
        assert!(inputs[1] != T::zero(), "Cannot divide by zero");
        inputs[0] / inputs[1]
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}
