use crate::engine::types::computation::component::ImplicitFunction;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Neovius {
    pub length_x: f32,
    pub length_y: f32,
    pub length_z: f32,
}

impl Neovius {
    pub fn new(length_x: f32, length_y: f32, length_z: f32) -> Self {
        Neovius {
            length_x: length_x,
            length_y: length_y,
            length_z: length_z,
        }
    }

    pub fn with_equal_spacing(length: f32) -> Self {
        Neovius {
            length_x: length,
            length_y: length,
            length_z: length,
        }
    }
}

impl ImplicitFunction for Neovius {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let x = 2.0 * PI * x / self.length_x;
        let y = 2.0 * PI * y / self.length_y;
        let z = 2.0 * PI * z / self.length_z;
        3.0*(x.cos()+y.cos()+z.cos())+4.0*x.cos()*y.cos()*z.cos()
    }
}
