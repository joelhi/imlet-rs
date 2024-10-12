use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::ImplicitOperation;

/// Operation to multiply two values -> a*b
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Multiply {}

/// Create a new Multiply operator
impl Default for Multiply {
    fn default() -> Self {
        Self::new()
    }
}

impl Multiply {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for Multiply {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] * inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to add two values -> a+b
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Add {}

impl Default for Add {
    fn default() -> Self {
        Self::new()
    }
}

impl Add {
    /// Create a new Add operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for Add {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to subtract a value from another -> a-b.
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Subtract {}

impl Default for Subtract {
    fn default() -> Self {
        Self::new()
    }
}

impl Subtract {
    /// Create a new Subtract operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for Subtract {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to divide a value with another -> a/b.
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Divide {}

impl Default for Divide {
    fn default() -> Self {
        Self::new()
    }
}

impl Divide {
    /// Create a new Divide operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for Divide {
    fn eval(&self, inputs: &[T]) -> T {
        debug_assert!(inputs[1] != T::zero(), "Cannot divide by zero");
        inputs[0] / inputs[1]
    }

    fn num_inputs(&self) -> usize {
        2
    }
}

/// Operation to perform a linear interpolation between two values -> a + t*(b-a).
///
/// This operation takes three inptus.
/// * First value to interpolate (a)
/// * Second value to interpolate (b)
/// * Interpolation parameter (t)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LinearInterpolation {}

impl Default for LinearInterpolation {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearInterpolation {
    /// Create a new LinearInterpolation operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: Float> ImplicitOperation<T> for LinearInterpolation {
    fn eval(&self, inputs: &[T]) -> T {
        let zero = T::zero();
        let one = T::one();
        let t = inputs[2].clamp(zero, one);
        inputs[0] + t * (inputs[1] - inputs[0])
    }

    fn num_inputs(&self) -> usize {
        3
    }
}
