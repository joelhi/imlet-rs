use std::fmt::{self, Display};

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::geometry::Vec3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Parameter {
    pub name: &'static str,
    pub data_type: DataType,
}

impl Parameter {
    pub fn new(name: &'static str, data_type: DataType) -> Self {
        Self { name, data_type }
    }

    pub fn set_value_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        param: &str,
        target: &mut T,
    ) -> bool {
        if let Data::Value(value) = data {
            if parameter_name == param {
                *target = *value;
                return true;
            }
        }
        false
    }

    pub fn set_clamped_value_from_param<T: Float>(
        parameter_name: &str,
        data: &Data<T>,
        param: &str,
        target: &mut T,
        min: T,
        max: T,
    ) -> bool {
        if let Data::Value(value) = data {
            if parameter_name == param {
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
        parameter_name: &String,
        data: &Data<T>,
        param: &str,
        target: &mut String,
    ) -> bool {
        if let Data::Text(value) = data {
            if parameter_name == param {
                *target = value.clone();
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    Value,
    Vec3,
    Boolean,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data<T> {
    Value(T),
    Vec3(Vec3<T>),
    Boolean(bool),
    Text(String),
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

    pub fn get_text(&self) -> Option<&String> {
        if let Data::Text(ref text) = self {
            Some(text)
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
            Data::Text(text) => write!(f, "{}", text),
        }
    }
}
