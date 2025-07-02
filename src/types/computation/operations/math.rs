use std::fmt::Debug;

use log::error;
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::{Data, DataType, Parameter},
    traits::{ImplicitComponent, ImplicitOperation, ModelFloat},
};

static INPUT_NAMES: [&str; 2] = ["First Number", "Second Number"];

/// Operation to multiply two values -> a*b
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Multiply {}

impl Default for Multiply {
    fn default() -> Self {
        Self::new()
    }
}

impl Multiply {
    /// Create a new Multiply operator
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
}

impl<T: Float> ImplicitComponent<T> for Multiply {
    fn name(&self) -> &'static str {
        "Multiply"
    }
}

/// Operation to add two values -> a+b
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
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
}

impl<T: Float> ImplicitComponent<T> for Add {
    fn name(&self) -> &'static str {
        "Add"
    }
}

/// Operation to subtract a value from another -> a-b.
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
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
}

impl<T> ImplicitComponent<T> for Subtract {
    fn name(&self) -> &'static str {
        "Subtract"
    }
}

/// Operation to divide a value with another -> a/b.
///
/// This operation takes two inputs.
/// * First value (a)
/// * Second value (b)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
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
}

impl<T: Float> ImplicitComponent<T> for Divide {
    fn name(&self) -> &'static str {
        "Divide"
    }
}

/// Operation to perform a linear interpolation between two values -> a + t*(b-a).
///
/// This operation takes two inptus.
/// * First value to interpolate (a)
/// * Second value to interpolate (b)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
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

impl<T: ModelFloat> ImplicitOperation<T> for LinearInterpolation<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + self.factor * (inputs[1] - inputs[0])
    }

    fn inputs(&self) -> &[&str] {
        LINEAR_INTERPOLATION_INPUTS
    }
}

impl<T: ModelFloat> ImplicitComponent<T> for LinearInterpolation<T> {
    fn parameters(&self) -> &[Parameter] {
        LINEAR_INTERPOLATION_PARAMETERS
    }

    fn name(&self) -> &'static str {
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

/// Operation to perform a linear interpolation between two values -> a + t*(b-a) with a variable interpolation function.
///
/// This operation takes three inptus.
/// * First value to interpolate (a)
/// * Second value to interpolate (b)
/// * Interpolation factor.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct VariableLinearInterpolation {}

static VAR_LINEAR_INTERPOLATION_INPUTS: &[&str] = &["First Value", "Second Value", "Factor"];

impl Default for VariableLinearInterpolation {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableLinearInterpolation {
    /// Create a new VariableLinearInterpolation operation.
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: ModelFloat> ImplicitOperation<T> for VariableLinearInterpolation {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] + inputs[2] * (inputs[1] - inputs[0])
    }

    fn inputs(&self) -> &[&str] {
        VAR_LINEAR_INTERPOLATION_INPUTS
    }
}

impl<T: ModelFloat> ImplicitComponent<T> for VariableLinearInterpolation {
    fn name(&self) -> &'static str {
        "VariableLinearInterpolation"
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Remap<T> {
    from_min: T,
    from_max: T,
    to_min: T,
    to_max: T,
}

static REMAP_PARAMETERS: &[Parameter] = &[
    Parameter {
        name: "FromMin",
        data_type: DataType::Value,
    },
    Parameter {
        name: "FromMax",
        data_type: DataType::Value,
    },
    Parameter {
        name: "ToMin",
        data_type: DataType::Value,
    },
    Parameter {
        name: "ToMax",
        data_type: DataType::Value,
    },
];

static REMAP_INPUTS: &[&str] = &["Value"];

impl<T: Float> Default for Remap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Float> Remap<T> {
    /// Create a new Remap operation with default range \[0,1\] -> \[0,1\].
    pub fn new() -> Self {
        Self {
            from_min: T::zero(),
            from_max: T::one(),
            to_min: T::zero(),
            to_max: T::one(),
        }
    }

    /// Create a new Remap operation with custom ranges.
    pub fn from_ranges(from_min: T, from_max: T, to_min: T, to_max: T) -> Self {
        Self {
            from_min,
            from_max,
            to_min,
            to_max,
        }
    }

    /// Create a new Remap operation to normalize a certain range.
    pub fn with_source_domain(from_min: T, from_max: T) -> Self {
        Self {
            from_min,
            from_max,
            to_min: T::zero(),
            to_max: T::one(),
        }
    }
}

impl<T: ModelFloat> ImplicitOperation<T> for Remap<T> {
    fn eval(&self, inputs: &[T]) -> T {
        let value = inputs[0];
        // First normalize to [0,1]
        let normalized = (value - self.from_min) / (self.from_max - self.from_min);
        // Then map to target range
        self.to_min + normalized * (self.to_max - self.to_min)
    }

    fn inputs(&self) -> &[&str] {
        REMAP_INPUTS
    }
}

impl<T: ModelFloat> ImplicitComponent<T> for Remap<T> {
    fn parameters(&self) -> &[Parameter] {
        REMAP_PARAMETERS
    }

    fn name(&self) -> &'static str {
        "Remap"
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        match parameter_name {
            "FromMin" => {
                let old_min = self.from_min;
                Parameter::set_value_from_param(
                    parameter_name,
                    &data,
                    "FromMin",
                    &mut self.from_min,
                );
                if (self.from_min - self.from_max).abs() < T::epsilon() {
                    error!("FromMin and FromMax can't be same value.");
                    self.from_min = old_min;
                }
            }
            "FromMax" => {
                let old_max = self.from_max;
                Parameter::set_value_from_param(
                    parameter_name,
                    &data,
                    "FromMax",
                    &mut self.from_max,
                );
                if (self.from_min - self.from_max).abs() < T::epsilon() {
                    error!("FromMin and FromMax can't be same value.");
                    self.from_max = old_max;
                }
            }
            "ToMin" => {
                let old_min = self.to_min;
                Parameter::set_value_from_param(parameter_name, &data, "ToMin", &mut self.to_min);
                if (self.to_min - self.to_max).abs() < T::epsilon() {
                    error!("ToMin and ToMax can't be same value.");
                    self.to_min = old_min;
                }
            }
            "ToMax" => {
                let old_max = self.to_max;
                Parameter::set_value_from_param(parameter_name, &data, "ToMax", &mut self.to_max);
                if (self.to_min - self.to_max).abs() < T::epsilon() {
                    error!("ToMin and ToMax can't be same value.");
                    self.to_max = old_max;
                }
            }
            _ => {
                error!("Unknown parameter name: {}", parameter_name);
            }
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "FromMin" => Some(Data::Value(self.from_min)),
            "FromMax" => Some(Data::Value(self.from_max)),
            "ToMin" => Some(Data::Value(self.to_min)),
            "ToMax" => Some(Data::Value(self.to_max)),
            _ => None,
        }
    }
}
