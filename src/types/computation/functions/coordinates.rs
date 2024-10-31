use log::error;
use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::ImplicitFunction;
use crate::types::computation::{Data, DataType, Parameter};
use crate::utils::math_helper::normalize;

static COORD_PARAMETERS: [Parameter; 2] = [
    Parameter {
        name: "Min",
        data_type: DataType::Value,
    },
    Parameter {
        name: "Max",
        data_type: DataType::Value,
    },
];

/// Distance function that evaluates to the z-coordinate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ZCoord<T> {
    min: T,
    max: T,
}

impl<T: Float> ZCoord<T> {
    /// Create a distance function for a remapped z domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular z coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for ZCoord<T> {
    fn eval(&self, _: T, _: T, z: T) -> T {
        normalize(z, self.min, self.max)
    }

    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min)
            || Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Min" => Some(Data::Value(self.min)),
            "Max" => Some(Data::Value(self.max)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Z Coord"
    }
}

/// Distance function that evaluates to the y-coordinate
#[derive(Debug, Clone, Copy)]
pub struct YCoord<T> {
    min: T,
    max: T,
}

impl<T: Float> YCoord<T> {
    /// Create a distance function for a remapped y domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular y coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for YCoord<T> {
    fn eval(&self, _: T, y: T, _: T) -> T {
        normalize(y, self.min, self.max)
    }

    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min)
            || Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Min" => Some(Data::Value(self.min)),
            "Max" => Some(Data::Value(self.max)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Y Coord"
    }
}

/// Distance function that evaluates to the x-coordinate
#[derive(Debug, Clone, Copy)]
pub struct XCoord<T> {
    min: T,
    max: T,
}

impl<T: Float> XCoord<T> {
    /// Create a distance function for a remapped x domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular x coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for XCoord<T> {
    fn eval(&self, x: T, _: T, _: T) -> T {
        normalize(x, self.min, self.max)
    }

    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min)
            || Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Min" => Some(Data::Value(self.min)),
            "Max" => Some(Data::Value(self.max)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "X Coord"
    }
}
