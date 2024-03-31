use std::f32::consts::PI;

use crate::engine::types::{ImplicitFunction, XYZ};

pub struct GyroidFunction {
    pub length_x: f32,
    pub length_y: f32,
    pub length_z: f32,
}

impl ImplicitFunction for GyroidFunction {
    fn eval(&self, x:f32, y:f32, z:f32)->f32 {
        (2.0 * PI * x / self.length_x).sin() * (2.0 * PI * y / self.length_y).cos()
            + (2.0 * PI * y / self.length_y).sin() * (2.0 * PI * z / self.length_z).cos()
            + (2.0 * PI * z / self.length_z).sin() * (2.0 * PI * x / self.length_x).cos()
    }
}

pub struct DistanceFunction {
    pub source: XYZ,
}

impl ImplicitFunction for DistanceFunction {
    fn eval(&self, x:f32, y:f32, z:f32)->f32 {
        self.source.distance_to_coord(x, y ,z)
    }
}