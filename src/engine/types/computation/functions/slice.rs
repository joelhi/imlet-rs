use crate::engine::types::{computation::component::ImplicitFunction, geometry::{Plane, Vec3f}};

#[derive(Debug, Clone, Copy)]
pub struct Slice {
    pub plane: Plane
}

impl Slice {
    pub fn new(plane: Plane) -> Self {
        Slice {
            plane: plane
        }
    }
}

impl ImplicitFunction for Slice {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.plane.signed_distance(Vec3f::new(x, y, z))
    }
}