use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{traits::ImplicitFunction, Data, DataType, Parameter},
    geometry::{Line, Vec3},
};

use super::traits::SignedDistance;

/// A capsule primitive defined by a line and a radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Capsule<T> {
    line: Line<T>,
    radius: T,
}

impl<T> Capsule<T> {
    /// Create a new Capsule.
    /// # Arguments
    ///
    /// * `line` -Capsule centre line.
    /// * `radius` - Capsule radius.
    pub fn new(line: Line<T>, radius: T) -> Self {
        Self { line, radius }
    }
    /// Creare a new capsule from start and end points.
    /// # Arguments
    ///
    /// * `start` - Start of line for capsule length.
    /// * `end` - End of line for capsule length.
    /// * `radius` - Capsule radius.
    pub fn from_points(start: Vec3<T>, end: Vec3<T>, radius: T) -> Self {
        Self {
            line: Line::new(start, end),
            radius,
        }
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Capsule<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.line.distance_to(Vec3::new(x, y, z)) - self.radius
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Capsule<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::new("Start", DataType::Vec3),
            Parameter::new("End", DataType::Vec3),
            Parameter::new("Radius", DataType::Value),
        ]
    }

    fn set_parameter(&mut self, parameter_name: &String, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Start", &mut self.line.start)
            || Parameter::set_vec3_from_param(parameter_name, &data, "End", &mut self.line.end)
            || Parameter::set_value_from_param(parameter_name, &data, "Radius", &mut self.radius))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &String) -> Option<Data<T>> {
        match parameter_name.as_str() {
            "Start" => Some(Data::Vec3(self.line.start)),
            "End" => Some(Data::Vec3(self.line.end)),
            "Radius" => Some(Data::Value(self.radius)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Capsule"
    }
}
