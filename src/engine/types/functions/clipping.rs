use crate::engine::types::{Plane, XYZ};

use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct ClippingPlane<T: ImplicitFunction>{
    pub function: T,
    pub plane: Plane
}


impl<T: ImplicitFunction> ImplicitFunction for ClippingPlane<T>{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        match self.plane.signed_distance(XYZ::new(x, y, z)){
            val if val < 0.0 => 0.0,
            _ => self.function.eval(x, y, z),
        }
    }
}