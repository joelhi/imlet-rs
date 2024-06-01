use crate::types::{computation::component::ImplicitFunction, geometry::Vec3f};

#[derive(Debug, Clone, Copy)]
pub struct Torus {
    pub center: Vec3f,
    pub r: f32,
    pub t: f32,
}

impl Torus {
    pub fn new(center: Vec3f, r: f32, t: f32) -> Self {
        Torus { center, r, t }
    }
}

impl ImplicitFunction for Torus {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.r - ((x - self.center.x).powi(2) + (z - self.center.z).powi(2)).sqrt()).powi(2)
            + (y - self.center.y).powi(2)
            - self.t.powi(2)
    }
}
