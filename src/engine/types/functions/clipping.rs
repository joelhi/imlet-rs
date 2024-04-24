use crate::engine::types::{Plane, XYZ};

use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct ClippingPlane<T: ImplicitFunction>{
    pub function: T,
    pub plane: Plane
}


impl<T: ImplicitFunction> ImplicitFunction for ClippingPlane<T>{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let plane_iso = self.plane.signed_distance(XYZ::new(x, y, z));
        let val = self.function.eval(x, y, z);
        plane_iso.max(val)
    }
}