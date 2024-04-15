use std::f32::consts::PI;

use crate::engine::types::{ImplicitFunction, XYZ};

pub struct GyroidFunction {
    pub length_x: f32,
    pub length_y: f32,
    pub length_z: f32,
}

impl ImplicitFunction for GyroidFunction {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        (2.0 * PI * x / self.length_x).sin() * (2.0 * PI * y / self.length_y).cos()
            + (2.0 * PI * y / self.length_y).sin() * (2.0 * PI * z / self.length_z).cos()
            + (2.0 * PI * z / self.length_z).sin() * (2.0 * PI * x / self.length_x).cos()
    }
}

pub struct Sphere {
    pub source: XYZ,
    pub radius: f32
}

impl ImplicitFunction for Sphere {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.source.distance_to_coord(x, y, z) - self.radius
    }
}

pub struct BitMask<T: ImplicitFunction>{
    pub function: T,
    pub cut_off: f32
}

impl<T: ImplicitFunction> ImplicitFunction for BitMask<T> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        match self.function.eval(x, y, z) {
            val if val > self.cut_off => 0.0,
            _ => 1.0,
        }
    }
}

pub struct ImplicitProduct<F, G> {
    pub f: F,
    pub g: G,
}

impl<F: ImplicitFunction, G: ImplicitFunction> ImplicitFunction for ImplicitProduct<F, G> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) * self.g.eval(x, y, z)
    }
}

pub struct Constant{
    pub value: f32
}

impl ImplicitFunction for Constant{
    fn eval(&self, _: f32, _: f32, _: f32) -> f32 {
        self.value
    }
}