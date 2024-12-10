use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{
        model::{Data, DataType, Parameter},
        traits::ImplicitFunction,
    },
    geometry::Vec3,
};

static TORUS_PARAMS: &[Parameter; 3] = &[
    Parameter {
        name: "Centre",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "Radius",
        data_type: DataType::Value,
    },
    Parameter {
        name: "Thickness",
        data_type: DataType::Value,
    },
];

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
    /// * `centre` - The centre point of the torus.
    /// * `r` - The major radius of the torus. This is the distance from the centre line to the centre of the torus.
    /// * `t` - The minor radius of the torus. This is the radius of the cross section.
    pub fn new(centre: Vec3<T>, r: T, t: T) -> Self {
        Torus { centre, r, t }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for Torus<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        let squared_value =
            (self.r - ((x - self.centre.x).powi(2) + (z - self.centre.z).powi(2)).sqrt()).powi(2)
                + (y - self.centre.y).powi(2)
                - self.t.powi(2);
        if squared_value < T::zero() {
            -(squared_value.abs().sqrt())
        } else {
            squared_value.sqrt()
        }
    }

    fn parameters(&self) -> &[Parameter] {
        TORUS_PARAMS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Centre", &mut self.centre)
            || Parameter::set_value_from_param(parameter_name, &data, "Radius", &mut self.r)
            || Parameter::set_value_from_param(parameter_name, &data, "Thickness", &mut self.t))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_centre_distance_value() {
        let torus = Torus::new(Vec3::origin(), 45., 5.);
        let val = torus.eval(45., 0., 0.);
        assert!(
            (val + 5.).abs() < f64::epsilon(),
            "Incorrect signed distance value at tours centre line, value was {}, but radius is {}",
            val,
            5.0
        );
    }
}
