use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{traits::ImplicitFunction, Data, DataType, Parameter};

use super::{traits::SignedDistance, Vec3};

/// Infinite plane, defined by origin point and normal direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Plane<T> {
    origin: Vec3<T>,
    normal: Vec3<T>,
}

impl<T: Float> Plane<T> {
    /// Create a new Plane from an origin point and a normal.
    ///
    /// # Arguments
    ///
    /// * `origin` - The location of the origin.
    /// * `normal` - The direction of the normal (z) direction.
    pub fn new(origin: Vec3<T>, normal: Vec3<T>) -> Self {
        Plane {
            origin,
            normal: normal.normalize(),
        }
    }

    /// Returns the origin of the plane.
    pub fn origin(&self) -> Vec3<T> {
        self.origin
    }

    /// Returns the normal of the plane.
    pub fn normal(&self) -> Vec3<T> {
        self.normal
    }

    /// Computes the signed distance to the plane from a point, based on x, y and z coordinates.
    ///
    /// Locations above the plane in the direction of the normal will return a positive distance. Locations below will be negative.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the point.
    /// * `y` - Y coordinate of the point.
    /// * `z` - Z coordinate of the point.
    pub fn signed_distance_coord(&self, x: T, y: T, z: T) -> T {
        self.normal
            .dot_coord(x - self.origin.x, y - self.origin.y, z - self.origin.z)
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Plane<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.signed_distance_coord(x, y, z)
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Plane<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance_coord(x, y, z)
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter::new("Origin", DataType::Vec3),
            Parameter::new("Normal", DataType::Vec3),
        ]
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Origin", &mut self.origin)
            || Parameter::set_vec3_from_param(parameter_name, &data, "Normal", &mut self.normal))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Origin" => Some(Data::Vec3(self.origin)),
            "Normal" => Some(Data::Vec3(self.normal)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "Torus"
    }
}
