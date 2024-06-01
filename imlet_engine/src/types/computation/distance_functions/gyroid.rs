use crate::types::computation::component::ImplicitFunction;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Gyroid {
    pub length_x: f32,
    pub length_y: f32,
    pub length_z: f32,
}

impl Gyroid {
    pub fn new(length_x: f32, length_y: f32, length_z: f32) -> Self {
        Gyroid {
            length_x: length_x,
            length_y: length_y,
            length_z: length_z,
        }
    }

    pub fn with_equal_spacing(length: f32) -> Self {
        Gyroid {
            length_x: length,
            length_y: length,
            length_z: length,
        }
    }
}

impl ImplicitFunction for Gyroid {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        (2.0 * PI * x / self.length_x).sin() * (2.0 * PI * y / self.length_y).cos()
            + (2.0 * PI * y / self.length_y).sin() * (2.0 * PI * z / self.length_z).cos()
            + (2.0 * PI * z / self.length_z).sin() * (2.0 * PI * x / self.length_x).cos()
    }
}
