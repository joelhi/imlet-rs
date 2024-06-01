use crate::types::{computation::component::ImplicitFunction, geometry::Vec3f};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub source: Vec3f,
    pub radius: f32,
}

impl Sphere {
    pub fn new(source: Vec3f, radius: f32)->Self{
        Sphere {
            source,
            radius,
        }
    }
}

impl ImplicitFunction for Sphere {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.source.distance_to_coord(x, y, z) - self.radius
    }
}