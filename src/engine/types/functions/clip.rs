use crate::engine::types::XYZ;

use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct ClippingPlane<T: ImplicitFunction>{
    pub function: T,
    pub direction: XYZ,
    pub distance: f32,
}


impl<T: ImplicitFunction> ImplicitFunction for ClippingPlane<T>{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        match self.direction.dot_xyz(x, y, z){
            val if val <= self.distance => 0.0,
            _ => self.function.eval(x, y, z),
        }
    }
}