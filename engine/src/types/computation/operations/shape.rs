use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::implicit_functions::ImplicitOperation;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Offset<T> {
    distance: T,
}

impl<T: Float + Debug> Offset<T> {
    pub fn new(offset_distance: T) -> Self {
        Self {
            distance: offset_distance,
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Offset<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - self.distance
    }

    fn num_inputs(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Thickness<T: Float + Debug> {
    t: T,
}

impl<T: Float + Debug> Thickness<T> {
    pub fn new(thickness: T) -> Self {
        Self {
            t: thickness,
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Thickness<T> {
    fn eval(&self, inputs: &[T]) -> T {
        let two = T::from(2.0).unwrap();
        (inputs[0] - self.t / two).max(-(inputs[0] + self.t / two))
    }

    fn num_inputs(&self) -> usize {
        1
    }
}
