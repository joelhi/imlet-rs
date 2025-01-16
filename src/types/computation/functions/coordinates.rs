use log::error;
use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::model::{Data, DataType, Parameter};
use crate::types::computation::traits::{ImplicitComponent, ImplicitFunction};
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
pub struct ZDomain<T> {
    min: T,
    max: T,
}

impl<T: Float> ZDomain<T> {
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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for ZDomain<T> {
    fn eval(&self, _: T, _: T, z: T) -> T {
        normalize(z, self.min, self.max)
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for ZDomain<T> {
    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if parameter_name == "Min" {
            let old_min = self.min;
            Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.min = old_min;
            }
        } else if parameter_name == "Max" {
            let old_max = self.max;
            Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.max = old_max;
            }
        } else {
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

    fn name(&self) -> &'static str {
        "ZDomain"
    }
}

/// Distance function that evaluates to the y-coordinate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct YDomain<T> {
    min: T,
    max: T,
}

impl<T: Float> YDomain<T> {
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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for YDomain<T> {
    fn eval(&self, _: T, y: T, _: T) -> T {
        normalize(y, self.min, self.max)
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for YDomain<T> {
    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if parameter_name == "Min" {
            let old_min = self.min;
            Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.min = old_min;
            }
        } else if parameter_name == "Max" {
            let old_max = self.max;
            Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.max = old_max;
            }
        } else {
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

    fn name(&self) -> &'static str {
        "YDomain"
    }
}

/// Distance function that evaluates to the x-coordinate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct XDomain<T> {
    min: T,
    max: T,
}

impl<T: Float> XDomain<T> {
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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for XDomain<T> {
    fn eval(&self, x: T, _: T, _: T) -> T {
        normalize(x, self.min, self.max)
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for XDomain<T> {
    fn parameters(&self) -> &[Parameter] {
        &COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if parameter_name == "Min" {
            let old_min = self.min;
            Parameter::set_value_from_param(parameter_name, &data, "Min", &mut self.min);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.min = old_min;
            }
        } else if parameter_name == "Max" {
            let old_max = self.max;
            Parameter::set_value_from_param(parameter_name, &data, "Max", &mut self.max);
            if (self.min - self.max).abs() < T::epsilon() {
                error!("Min and max can't be same value.");
                self.max = old_max;
            }
        } else {
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

    fn name(&self) -> &'static str {
        "XDomain"
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CoordinateValue {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct XYZValue {
    coordinate_value: CoordinateValue,
}

const GLOBAL_COORD_PARAMETERS: [Parameter; 1] = [Parameter {
    name: "Coordinate",
    data_type: DataType::Enum(&["X", "Y", "Z"]),
}];

impl XYZValue {
    pub fn new(coordinate_value: CoordinateValue) -> Self {
        Self { coordinate_value }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for XYZValue {
    fn eval(&self, x: T, y: T, z: T) -> T {
        match self.coordinate_value {
            CoordinateValue::X => x,
            CoordinateValue::Y => y,
            CoordinateValue::Z => z,
        }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for XYZValue {
    fn parameters(&self) -> &[Parameter] {
        &GLOBAL_COORD_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if let Some(selection) =
            Parameter::get_string_from_enum_param(parameter_name, &data, "Coordinate")
        {
            match selection.as_str() {
                "X" => {
                    self.coordinate_value = CoordinateValue::X;
                }
                "Y" => {
                    self.coordinate_value = CoordinateValue::Y;
                }
                "Z" => {
                    self.coordinate_value = CoordinateValue::Z;
                }
                _ => (),
            };
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Coordinate" => match self.coordinate_value {
                CoordinateValue::X => Some(Data::EnumValue("X".to_string())),
                CoordinateValue::Y => Some(Data::EnumValue("Y".to_string())),
                CoordinateValue::Z => Some(Data::EnumValue("Z".to_string())),
            },
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "XYZValue"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_z_domain() {
        let domain: ZDomain<f32> = ZDomain::natural();

        assert!(domain.eval(1., 0., 0.).abs() < f32::epsilon());
        assert!((domain.eval(1., 0., 0.5) - 0.5).abs() < f32::epsilon());
        assert!((domain.eval(1., 0., 1.) - 1.).abs() < f32::epsilon());
    }

    #[test]
    fn test_natural_y_domain() {
        let domain: YDomain<f32> = YDomain::natural();

        assert!(domain.eval(1., 0., 0.).abs() < f32::epsilon());
        assert!((domain.eval(1., 0.5, 0.5) - 0.5).abs() < f32::epsilon());
        assert!((domain.eval(1., 1., 1.) - 1.).abs() < f32::epsilon());
    }

    #[test]
    fn test_natural_x_domain() {
        let domain: XDomain<f32> = XDomain::natural();

        assert!(domain.eval(0., 1., 0.).abs() < f32::epsilon());
        assert!((domain.eval(0.5, 0., 1.0) - 0.5).abs() < f32::epsilon());
        assert!((domain.eval(1., 1., 1.) - 1.).abs() < f32::epsilon());
    }

    #[test]
    fn test_handle_zero_size_domain_x() {
        let mut domain: XDomain<f32> = XDomain::natural();

        domain.set_parameter("Max", Data::Value(0.0));

        //Check that the parameter is still 1.0
        assert!((domain.max - 1.0).abs() < f32::epsilon());

        domain.set_parameter("Min", Data::Value(1.0));

        //Check that the parameter is still 0.0
        assert!((domain.min).abs() < f32::epsilon());
    }

    #[test]
    fn test_handle_zero_size_domain_y() {
        let mut domain: YDomain<f32> = YDomain::natural();

        domain.set_parameter("Max", Data::Value(0.0));

        //Check that the parameter is still 1.0
        assert!(
            (domain.max - 1.0).abs() < f32::epsilon(),
            "Invalid value assigned. Should be 1.0 but was {}",
            domain.max
        );

        domain.set_parameter("Min", Data::Value(1.0));

        //Check that the parameter is still 0.0
        assert!(
            (domain.min).abs() < f32::epsilon(),
            "Invalid value assigned. Should be 0.0 but was {}",
            domain.min
        );
    }

    #[test]
    fn test_handle_zero_size_domain_z() {
        let mut domain: ZDomain<f32> = ZDomain::natural();

        domain.set_parameter("Max", Data::Value(0.0));

        //Check that the parameter is still 1.0
        assert!((domain.max - 1.0).abs() < f32::epsilon());

        domain.set_parameter("Min", Data::Value(1.0));

        //Check that the parameter is still 0.0
        assert!((domain.min).abs() < f32::epsilon());
    }
}
