use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{traits::ImplicitFunction, Data, DataType, Parameter},
    geometry::Vec3,
};

/// Distance function for a torus, defined by an a centre point, major radius and minor radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Torus<T> {
    /// The centre point
    pub centre: Vec3<T>,
    /// Major radius of the torus
    pub r: T,
    /// Minor radius of the torus
    pub t: T,
}

impl<T> Torus<T> {
    /// Create a new sphere.
    /// # Arguments
    ///
    /// * `centre` -The centre point of the torus.
    /// * `r` -The major radius of the torus. This is the distance from the centre line to the centre of the torus.
    /// * `t` -The minor radius of the torus. This is the radius of the cross section.
    pub fn new(centre: Vec3<T>, r: T, t: T) -> Self {
        Torus { centre, r, t }
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Torus<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        (self.r - ((x - self.centre.x).powi(2) + (z - self.centre.z).powi(2)).sqrt()).powi(2)
            + (y - self.centre.y).powi(2)
            - self.t.powi(2)
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::new("Centre", DataType::Value),
            Parameter::new("Radius", DataType::Value),
            Parameter::new("Thickness", DataType::Value),
        ]
    }

    fn set_parameter(&mut self, parameter_name: &String, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Centre", &mut self.centre)
            || Parameter::set_value_from_param(parameter_name, &data, "Radius", &mut self.r)
            || Parameter::set_value_from_param(parameter_name, &data, "Thickness", &mut self.t))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &String) -> Option<Data<T>> {
        match parameter_name.as_str() {
            "Centre" => Some(Data::Vec3(self.centre)),
            "Radius" => Some(Data::Value(self.r)),
            "Thickness" => Some(Data::Value(self.t)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Torus"
    }
}
