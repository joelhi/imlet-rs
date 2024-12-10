use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::Data, model::DataType, model::Parameter, traits::ImplicitOperation,
};

static INPUT_NAMES: [&str; 2] = ["First Number", "Second Number"];

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

    fn inputs(&self) -> &[&str] {
        &INPUT_NAMES
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {}

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn operation_name(&self) -> &'static str {
        "Multiply"
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

    fn inputs(&self) -> &[&str] {
        &INPUT_NAMES
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {}

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn operation_name(&self) -> &'static str {
        "Add"
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

    fn inputs(&self) -> &[&str] {
        &INPUT_NAMES
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {}

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn operation_name(&self) -> &'static str {
        "Subtract"
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

    fn inputs(&self) -> &[&str] {
        &INPUT_NAMES
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {}

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn operation_name(&self) -> &'static str {
        "Divide"
    }
}

/// Operation to perform a linear interpolation between two values -> a + t*(b-a).
///
/// This operation takes three inptus.
/// * First value to interpolate (a)
/// * Second value to interpolate (b)
/// * Interpolation parameter (t)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearInterpolation<T> {
    factor: T,
}

static LINEAR_INTERPOLATION_PARAMETERS: &[Parameter] = &[Parameter {
    name: "Factor",
    data_type: DataType::Value,
}];

static LINEAR_INTERPOLATION_INPUTS: &[&str] = &["First Value", "Second Value"];

impl<T: Float> Default for LinearInterpolation<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Float> LinearInterpolation<T> {
    /// Create a new LinearInterpolation operation.
    pub fn new() -> Self {
        Self {
            factor: T::from(0.5).expect("Should be able to conver 0,5 to T"),
        }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitOperation<T> for LinearInterpolation<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + self.factor * (inputs[1] - inputs[0])
    }

    fn inputs(&self) -> &[&str] {
        LINEAR_INTERPOLATION_INPUTS
    }

    fn parameters(&self) -> &[Parameter] {
        LINEAR_INTERPOLATION_PARAMETERS
    }

    fn operation_name(&self) -> &'static str {
        "LinearInterpolation"
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_clamped_value_from_param(
            parameter_name,
            &data,
            "Factor",
            &mut self.factor,
            T::zero(),
            T::one(),
        )) {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Factor" => Some(Data::Value(self.factor)),
            _ => None,
        }
    }
}
