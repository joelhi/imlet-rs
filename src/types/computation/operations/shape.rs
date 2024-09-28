use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::ImplicitOperation;

/// Operation to perform a boolean union on two distance values -> min(a, b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanUnion {}

impl BooleanUnion {
    /// Create a new BooleanUnion operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for BooleanUnion {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].min(inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to perform a boolean intersection on two distance values -> max(a, b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanIntersection {}

impl BooleanIntersection {
    /// Create a new BooleanIntersection operation
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for BooleanIntersection {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to perform a boolean difference on two distance values -> max(a, -b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanDifference {}

impl BooleanDifference {
    /// Create a new BooleanDifference operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for BooleanDifference {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(-inputs[1])
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to perform an offset on a distance value, which is the same as the subtraction operation -> a - distance.
///
/// This function takes one input.
/// * Distance value (a)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Offset<T> {
    distance: T,
}

impl<T> Offset<T> {
    /// Create a new Offset operation.
    ///
    /// # Arguments
    ///
    /// * `offset_distance` - The distance this operation will offset with.
    pub fn new(offset_distance: T) -> Self {
        Self {
            distance: offset_distance,
        }
    }
}

impl<T: Float + Send + Sync> ImplicitOperation<T> for Offset<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - self.distance
    }

    fn num_inputs(&self) -> usize {
        1
    }
}

/// Operation to give thickness to a surface. This will offset the input outwards and inwards by half the thickness and perform a difference operation.
///
/// This function takes one input.
/// * Thickness value
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Thickness<T> {
    t: T,
}

impl<T> Thickness<T> {
    /// Create a new Thickness operation.
    ///
    /// # Arguments
    ///
    /// * `thickness` - The thickness distance this will apply to the input.
    pub fn new(thickness: T) -> Self {
        Self { t: thickness }
    }
}

impl<T: Float + Send + Sync> ImplicitOperation<T> for Thickness<T> {
    fn eval(&self, inputs: &[T]) -> T {
        let two = T::from(2.0).unwrap();
        (inputs[0] - self.t / two).max(-(inputs[0] + self.t / two))
    }

    fn num_inputs(&self) -> usize {
        1
    }
}
