use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    types::computation::{
        components::{Data, DataType, Parameter},
        traits::ImplicitFunction,
    },
    utils::math_helper::Pi,
};
use std::fmt::Debug;

/// Function representing an approximate distance function for a gyroid surface.
///
/// This fuction is not a perfect distance function, and values deviate slightly from the true distance away from the surface.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gyroid<T> {
    pub length_x: T,
    pub length_y: T,
    pub length_z: T,
    pub linear: bool,
}

impl<T: Float> Gyroid<T> {
    /// Create a new gyroid function with custom period lengths in x, y and z directions.
    /// # Arguments
    ///
    /// * `length_x` -The length of one period (cell size) in x-direction.
    /// * `length_y` -The length of one period (cell size) in y-direction.
    /// * `length_z` -The length of one period (cell size) in z-direction.
    /// * `linear` - The gyrioid function is nonlinear in nature. Use this option to linearize the values.
    pub fn new(length_x: T, length_y: T, length_z: T, linear: bool) -> Self {
        Self {
            length_x,
            length_y,
            length_z,
            linear,
        }
    }

    /// Create a new gyroid function with equal period lengths in x, y and z directions.
    /// # Arguments
    ///
    /// * `length` -The length of one period (cell size) in all directions.
    /// * `linear` - The gyrioid function is nonlinear in nature. Use this option to linearize the values.
    pub fn with_equal_spacing(length: T, linear: bool) -> Self {
        Self {
            length_x: length,
            length_y: length,
            length_z: length,
            linear,
        }
    }
}

static GYROID_PARAMETERS: &[Parameter; 4] = &[
    Parameter {
        name: "Length X",
        data_type: DataType::Value,
    },
    Parameter {
        name: "Length Y",
        data_type: DataType::Value,
    },
    Parameter {
        name: "Length Z",
        data_type: DataType::Value,
    },
    Parameter {
        name: "Linearize",
        data_type: DataType::Boolean,
    },
];

impl<T: Pi + Float + Send + Sync> ImplicitFunction<T> for Gyroid<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let two = T::from(2.0).unwrap();
        let normalized_distance = (T::pi() * x / self.length_x).sin()
            * (T::pi() * y / self.length_y).cos()
            + (T::pi() * y / self.length_y).sin() * (T::pi() * z / self.length_z).cos()
            + (T::pi() * z / self.length_z).sin() * (T::pi() * x / self.length_x).cos();

        let scale = self.length_x.min(self.length_y).min(self.length_z) / two;

        if self.linear {
            let linear_distance =
                normalized_distance.clamp(-T::one(), T::one()).asin() / (T::pi() / two);
            scale * linear_distance
        } else {
            scale * normalized_distance
        }
    }

    fn parameters(&self) -> &[Parameter] {
        GYROID_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Length X", &mut self.length_x)
            || Parameter::set_value_from_param(
                parameter_name,
                &data,
                "Length Y",
                &mut self.length_y,
            )
            || Parameter::set_value_from_param(
                parameter_name,
                &data,
                "Length Z",
                &mut self.length_z,
            )
            || Parameter::set_bool_from_param(parameter_name, &data, "Linearize", &mut self.linear))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Length X" => Some(Data::Value(self.length_x)),
            "Length Y" => Some(Data::Value(self.length_y)),
            "Length Z" => Some(Data::Value(self.length_z)),
            "Linearize" => Some(Data::Boolean(self.linear)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Gyroid"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compute_linearized() {
        let linear_gyroid: Gyroid<f64> = Gyroid::with_equal_spacing(1.5, true);

        assert!(linear_gyroid.eval(0.0, 0.0, 0.0).abs() < 0.001);
        assert!((linear_gyroid.eval(0.75, 0.0, 0.0) - 0.75).abs() < 0.001);
        assert!((linear_gyroid.eval(1.5, 0.0, 0.0)).abs() < 0.001);

        assert!((linear_gyroid.eval(0.0, 0.75, 0.0) - 0.75).abs() < 0.001);
        assert!((linear_gyroid.eval(0.0, 1.5, 0.0)).abs() < 0.001);

        assert!((linear_gyroid.eval(0.0, 0.0, 0.75) - 0.75).abs() < 0.001);
        assert!((linear_gyroid.eval(0.0, 0.0, 1.5)).abs() < 0.001);

        assert!((linear_gyroid.eval(1.5, 1.5, 1.5)).abs() < 0.001);
    }
}
