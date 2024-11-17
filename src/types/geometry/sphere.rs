use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{
        components::{Data, DataType, Parameter},
        traits::ImplicitFunction,
    },
    geometry::Vec3,
};

use super::{traits::SignedDistance, BoundingBox};

static SPHERE_PARAMS: &[Parameter; 2] = &[
    Parameter {
        name: "Centre",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "Radius",
        data_type: DataType::Value,
    },
];

/// A sphere object, defined by a centre point and a radius.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sphere<T> {
    pub centre: Vec3<T>,
    pub radius: T,
}

impl<T> Sphere<T> {
    /// Create a new sphere.
    /// # Arguments
    ///
    /// * `centre` -The centre point of the sphere.
    /// * `radius` -The radius of the sphere
    pub fn new(centre: Vec3<T>, radius: T) -> Self {
        Self { centre, radius }
    }

    /// Create a new sphere at a point defined by x, y and z coordinates.
    /// # Arguments
    ///
    /// * `x` -The x coord of the centre of the sphere.
    /// * `y` -The y coord of the centre of the sphere.
    /// * `z` -The z coord of the centre of the sphere.
    /// * `radius` -The radius of the sphere
    pub fn at_coord(x: T, y: T, z: T, radius: T) -> Self {
        Self {
            centre: Vec3::new(x, y, z),
            radius,
        }
    }
}

impl<T: Float> Sphere<T> {
    /// Compute the bounding box for the sphere
    pub fn bounds(&self) -> BoundingBox<T> {
        let min = self.centre - Vec3::new(self.radius, self.radius, self.radius);
        let max = self.centre + Vec3::new(self.radius, self.radius, self.radius);
        BoundingBox::new(min, max)
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Sphere<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.centre.distance_to_coord(x, y, z) - self.radius
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for Sphere<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.centre.distance_to_coord(x, y, z) - self.radius
    }

    fn parameters(&self) -> &[Parameter] {
        SPHERE_PARAMS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Centre", &mut self.centre)
            || Parameter::set_value_from_param(parameter_name, &data, "Radius", &mut self.radius))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Centre" => Some(Data::Vec3(self.centre)),
            "Radius" => Some(Data::Value(self.radius)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Sphere"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_assigns_params() {
        let mut sphere = Sphere::new(Vec3::new(1., 1., 1.), 10.);

        let params: Vec<Parameter> = sphere.parameters().iter().map(|p| p.clone()).collect();

        for param in params {
            match param.data_type {
                DataType::Value => sphere.set_parameter(&param.name, Data::Value(1.)),
                DataType::Vec3 => {
                    sphere.set_parameter(&param.name, Data::Vec3(Vec3::new(1., 1., 1.)))
                }
                _ => panic!("Error in the param"),
            }
        }

        assert!((sphere.radius - 1.).abs() < f64::epsilon());
        assert!(sphere.centre.distance_to_coord(1., 1., 1.).abs() < f64::epsilon());
    }
}
