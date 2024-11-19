use std::fmt::{self, Display};

use num_traits::Float;
use serde::{Deserialize, Serialize};
use crate::types::geometry::Vec3;

/// Defines an input parameter to change the value of an [`crate::types::computation::traits::ImplicitFunction`] or [`crate::types::computation::traits::ImplicitOperation`].
/// 
/// This offers a public mechanism to change the internal values of functions at runtime. The parameters are defined by a name and a [`DataType`].
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Display name of the parameter.
    pub name: &'static str,
    /// Datatype of the parameter.
    pub data_type: DataType,
}

impl Parameter {
    /// Create a new parameter from a name and a [`DataType`]
    pub fn new(name: &'static str, data_type: DataType) -> Self {
        Self { name, data_type }
    }

    /// Helper method to assign the value from a matching parameter to a floating point variable. 
    /// It will assign the value of the parameter to the target input if the parameter_name matches the target_name and the data provided matches the target type, which in this instance is a Float value.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the provided parameter
    /// * `data` - The data contained.
    /// * `target_name` - The name of the current target.
    /// * `target` - Where the data should be assigned if matching the name and the type.
    pub fn set_value_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        target_name: &str,
        target: &mut T,
    ) -> bool {
        if let Data::Value(value) = data {
            if parameter_name == target_name {
                *target = *value;
                return true;
            }
        }
        false
    }

    /// Helper method to get the value from a matching parameter for Enum data. 
    /// 
    /// This method will return the string value contained in the parameter if the provided data is of enum type and the name matches the target name.
    /// Otherwise it will return [`None`].
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the provided parameter
    /// * `data` - The data contained in the parameter.
    /// * `target_name` - The name of the current target.
    pub fn get_string_from_enum_param<T>(
        parameter_name: &str,
        data: &Data<T>,
        target_name: &str,
    ) -> Option<String> {
        if let Data::EnumValue(value) = data {
            if parameter_name == target_name {
                return Some(value.clone());
            }
        }
        None
    }

    /// Helper method to assign the value from a matching parameter to a field. 
    /// It will assign the value of the parameter to the target input if the parameter_name matches the target_name and the data provided matches the target type, which in this instance is a Float value.
    /// 
    /// The difference between this and [`set_value_from_param`] is that this method will ensure the value, if assigned, is kept within the bounds.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the provided parameter
    /// * `data` - The data contained.
    /// * `target_name` - The name of the current target.
    /// * `target` - Where the data should be assigned if matching the name and the type.
    /// * `min` - Minimum allowed value. If the provided value is smaller than this, the min value will be assigned.
    /// * `max` - Maximum allowed value. If the provided value is larger than this, the max value will be assigned.
    pub fn set_clamped_value_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        target_name: &str,
        target: &mut T,
        min: T,
        max: T,
    ) -> bool {
        if let Data::Value(value) = data {
            if parameter_name == target_name {
                *target = value.clamp(min, max);
                return true;
            }
        }
        false
    }

    pub fn set_vec3_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        param: &str,
        target: &mut Vec3<T>,
    ) -> bool {
        if let Data::Vec3(value) = data {
            if parameter_name == param {
                *target = *value;
                return true;
            }
        }
        false
    }

    pub fn set_bool_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        param: &str,
        target: &mut bool,
    ) -> bool {
        if let Data::Boolean(value) = data {
            if parameter_name == param {
                *target = *value;
                return true;
            }
        }
        false
    }

    pub fn set_text_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        param: &str,
        target: &mut String,
    ) -> bool {
        if let Data::File(value) = data {
            if parameter_name == param {
                *target = value.clone();
                return true;
            }
        }
        false
    }
}

/// Enum to declare the data types which can be passed into the public parameters.
/// 
/// This enum holds no data, and only specifies the type of data for a [`Parameter`]. The enum which is used to pass data is called [`Data`].
#[derive(Debug, Clone)]
pub enum DataType {
    /// A floating point value. Can be [`f32`] or [`f64`] 
    Value,
    /// A a 3-dimensional coordinate. Passed as a [`Vec3`]
    Vec3,
    /// A [`bool`] which can be either true or false.
    Boolean,
    /// An uncontrained string. This can be any text.
    Text,
    /// A constrained selection set, defined as a list of possible options.
    Enum(&'static [&'static str]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data<T> {
    Value(T),
    Vec3(Vec3<T>),
    Boolean(bool),
    File(String),
    EnumValue(String),
}

impl<T> Data<T> {
    pub fn get_value(&self) -> Option<&T> {
        if let Data::Value(ref value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_vec3(&self) -> Option<&Vec3<T>> {
        if let Data::Vec3(ref vec3) = self {
            Some(vec3)
        } else {
            None
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        if let Data::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn get_file(&self) -> Option<String> {
        if let Data::File(ref path) = self {
            Some(path.clone())
        } else {
            None
        }
    }
}

// Implement the Display trait for Data<T>
impl<T: Display> Display for Data<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Value(value) => write!(f, "{}", value),
            Data::Vec3(vec3) => write!(f, "({}, {}, {})", vec3.x, vec3.y, vec3.z),
            Data::Boolean(b) => write!(f, "{}", b),
            Data::File(path) => write!(f, "File: {}", path),
            Data::EnumValue(text) => write!(f, "Selection: {}", text),
        }
    }
}
