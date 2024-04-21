use crate::engine::types::XYZ;

use super::ImplicitFunction;

pub struct Line{
    start: XYZ,
    end: XYZ,
    radius: f32,
}

impl Line {
    pub fn new(start: XYZ, end: XYZ, radius: f32)->Self{
        Line { start: start, end: end, radius: radius }
    }
}

impl ImplicitFunction for Line{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let pt = XYZ::new(x, y, z);
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(v2)).clamp(0.0, self.start.distance_to_xyz(self.end));
        let pt_on_line = self.start + (t * v2);
        pt_on_line.distance_to_xyz(pt) - self.radius 
    }
}