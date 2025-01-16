use std::fmt::Debug;

use log::error;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::{Data, DataType, Parameter}, traits::{ImplicitComponent, ImplicitFunction},
};

use super::{traits::SignedDistance, Vec3};

/// Infinite plane, defined by origin point and normal direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Plane<T> {
    origin: Vec3<T>,
    normal: Vec3<T>,
}

static PLANE_PARAMS: &[Parameter; 2] = &[
    Parameter {
        name: "Origin",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "Normal",
        data_type: DataType::Vec3,
    },
];

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

    /// Create a new global XY plane at the origin point `{0,0,0}`
    ///
    /// # Arguments
    ///
    /// * `origin` - The location of the origin.
    /// * `normal` - The direction of the normal (z) direction.
    pub fn xy() -> Self {
        Plane {
            origin: Vec3::origin(),
            normal: Vec3::z_axis().normalize(),
        }
    }

    /// Create a new global YZ plane at the origin point `{0,0,0}`
    pub fn yz() -> Self {
        Plane {
            origin: Vec3::origin(),
            normal: Vec3::x_axis().normalize(),
        }
    }

    /// Create a new global XZ plane at the origin point `{0,0,0}`
    pub fn xz() -> Self {
        Plane {
            origin: Vec3::origin(),
            normal: Vec3::y_axis().normalize(),
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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for Plane<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance_coord(x, y, z)
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for Plane<T>{
    fn parameters(&self) -> &[Parameter] {
        PLANE_PARAMS
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

    fn name(&self) -> &'static str {
        "Plane"
    }
}
