use std::fmt::Debug;

use log::error;
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::{Data, DataType, Parameter},
    traits::{ImplicitComponent, ImplicitFunction, ModelFloat},
};

use super::{traits::SignedDistance, Vec3};

static LINE_PARAMS: &[Parameter; 2] = &[
    Parameter {
        name: "Start",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "End",
        data_type: DataType::Vec3,
    },
];

/// A line segment defined by start and end points.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Line<T> {
    pub start: Vec3<T>,
    pub end: Vec3<T>,
}

impl<T> Line<T> {
    /// Create a new Line from a start and end point.
    ///
    /// # Arguments
    ///
    /// * `start` - Start of the line.
    /// * `end` - End of the line.
    pub fn new(start: Vec3<T>, end: Vec3<T>) -> Self {
        Self { start, end }
    }
}

impl<T: Float> Line<T> {
    /// Computes the distance to the closest point on the line from a point.
    ///
    /// # Arguments
    ///
    /// * `point` - Point from which distance is computed.
    pub fn distance_to(&self, point: Vec3<T>) -> T {
        self.closest_pt(point).distance_to_vec3(&point)
    }

    /// Computes the closest point on the line from a point.
    ///
    /// # Arguments
    ///
    /// * `point` - Point from which the closest point is computed.
    pub fn closest_pt(&self, pt: Vec3<T>) -> Vec3<T> {
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(&v2)).clamp(T::zero(), self.start.distance_to_vec3(&self.end));
        self.start + (v2 * t)
    }

    /// Compute the length of the line.
    pub fn length(&self) -> T {
        self.start.distance_to_vec3(&self.end)
    }
}

impl<T: ModelFloat> SignedDistance<T> for Line<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.distance_to(Vec3::new(x, y, z))
    }
}

impl<T: ModelFloat> ImplicitFunction<T> for Line<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }
}

impl<T: ModelFloat> ImplicitComponent<T> for Line<T> {
    fn parameters(&self) -> &[Parameter] {
        LINE_PARAMS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Start", &mut self.start)
            || Parameter::set_vec3_from_param(parameter_name, &data, "End", &mut self.end))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Start" => Some(Data::Vec3(self.start)),
            "End" => Some(Data::Vec3(self.end)),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "Line"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_assigns_params() {
        let mut line = Line::new(Vec3::new(1., 1., 1.), Vec3::new(10., 10., 10.));

        let params: Vec<&str> = line.parameters().iter().map(|p| p.name).collect();

        for param in params {
            line.set_parameter(param, Data::Vec3(Vec3::origin()));
        }

        assert!(line.length().abs() < f64::epsilon());
    }
}
