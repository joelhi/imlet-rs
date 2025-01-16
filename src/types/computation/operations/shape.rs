use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::{Data, DataType, Parameter},
    traits::{ImplicitComponent, ImplicitOperation},
};

/// Operation to perform a boolean union on two distance values -> min(a, b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanUnion {}

impl Default for BooleanUnion {
    fn default() -> Self {
        Self::new()
    }
}

impl BooleanUnion {
    /// Create a new BooleanUnion operation.
    pub fn new() -> Self {
        Self {}
    }
}

static UNION_INPUT_NAMES: [&str; 2] = ["Shape A", "Shape B"];

impl<T: Float> ImplicitOperation<T> for BooleanUnion {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].min(inputs[1])
    }

    fn inputs(&self) -> &[&str] {
        &UNION_INPUT_NAMES
    }
}

impl<T> ImplicitComponent<T> for BooleanUnion{
    fn name(&self) -> &'static str {
        "BooleanUnion"
    }
}

/// Operation to perform a boolean intersection on two distance values -> max(a, b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanIntersection {}

impl Default for BooleanIntersection {
    fn default() -> Self {
        Self::new()
    }
}

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

    fn inputs(&self) -> &[&str] {
        &UNION_INPUT_NAMES
    }
}

impl<T: Float> ImplicitComponent<T> for BooleanIntersection {
    fn name(&self) -> &'static str {
        "BooleanIntersection"
    }
}

/// Operation to perform a boolean difference on two distance values -> max(a, -b)
///
/// This function takes two inputs.
/// * First distance value (a)
/// * Second distance value (b)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanDifference {}

impl Default for BooleanDifference {
    fn default() -> Self {
        Self::new()
    }
}

impl BooleanDifference {
    /// Create a new BooleanDifference operation.
    pub fn new() -> Self {
        Self {}
    }
}

static DIFF_INPUT_NAMES: [&str; 2] = ["Base Shape", "Subtract Shape"];

impl<T: Float> ImplicitOperation<T> for BooleanDifference {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0].max(-inputs[1])
    }

    fn inputs(&self) -> &[&str] {
        &DIFF_INPUT_NAMES
    }
}

impl<T: Float> ImplicitComponent<T> for BooleanDifference {
    fn name(&self) -> &'static str {
        "BooleanDifference"
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

static OFFSET_INPUT_NAMES: &[&str; 1] = &["Shape"];

static OFFSET_PARAMETERS: &[Parameter] = &[Parameter {
    name: "Distance",
    data_type: DataType::Value,
}];

impl<T: Float + Send + Sync + Serialize> ImplicitOperation<T> for Offset<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - self.distance
    }

    fn inputs(&self) -> &[&str] {
        OFFSET_INPUT_NAMES
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for Offset<T> {
    fn parameters(&self) -> &[Parameter] {
        OFFSET_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Distance", &mut self.distance))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Distance" => Some(Data::Value(self.distance)),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Offset"
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

static THICKNESS_INPUT_NAMES: [&str; 1] = ["Shape"];

static THICKNESS_PARAMETERS: &[Parameter] = &[Parameter {
    name: "Thickness",
    data_type: DataType::Value,
}];

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

impl<T: Float + Send + Sync + Serialize> ImplicitOperation<T> for Thickness<T> {
    fn eval(&self, inputs: &[T]) -> T {
        let two = T::from(2.0).unwrap();
        (inputs[0] - self.t / two).max(-(inputs[0] + self.t / two))
    }

    fn inputs(&self) -> &[&str] {
        &THICKNESS_INPUT_NAMES
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for Thickness<T>{
    fn parameters(&self) -> &[Parameter] {
        THICKNESS_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Thickness", &mut self.t)) {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Thickness" => Some(Data::Value(self.t)),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Thickness"
    }
}
