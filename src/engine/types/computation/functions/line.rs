use crate::engine::types::{computation::component::ImplicitFunction, geometry::Vec3f};

pub struct Line{
    start: Vec3f,
    end: Vec3f,
    radius: f32,
}

impl Line {
    pub fn new(start: Vec3f, end: Vec3f, radius: f32)->Self{
        Line { start: start, end: end, radius: radius }
    }
}

impl ImplicitFunction for Line{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let pt = Vec3f::new(x, y, z);
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(v2)).clamp(0.0, self.start.distance_to_vec3f(self.end));
        let pt_on_line = self.start + (t * v2);
        pt_on_line.distance_to_vec3f(pt) - self.radius 
    }
}