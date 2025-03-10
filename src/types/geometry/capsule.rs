use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{
        model::{Data, DataType, Parameter},
        traits::{ImplicitComponent, ImplicitFunction},
    },
    geometry::{Line, Vec3},
};

use super::traits::SignedDistance;

static CAPSULE_PARAMS: &[Parameter; 3] = &[
    Parameter {
        name: "Start",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "End",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "Radius",
        data_type: DataType::Value,
    },
];

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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for Capsule<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for Capsule<T> {
    fn parameters(&self) -> &[Parameter] {
        CAPSULE_PARAMS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Start", &mut self.line.start)
            || Parameter::set_vec3_from_param(parameter_name, &data, "End", &mut self.line.end)
            || Parameter::set_value_from_param(parameter_name, &data, "Radius", &mut self.radius))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Start" => Some(Data::Vec3(self.line.start)),
            "End" => Some(Data::Vec3(self.line.end)),
            "Radius" => Some(Data::Value(self.radius)),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Capsule"
    }
}
