use std::fmt::Debug;

use num_traits::Float;

use crate::types::{computation::component::ImplicitFunction, geometry::Vec3};

pub struct Line<T: Float + Debug>{
    start: Vec3<T>,
    end: Vec3<T>,
    radius: T,
}

impl<T: Float + Debug> Line<T> {
    pub fn new(start: Vec3<T>, end: Vec3<T>, radius: T)->Self{
        Line { start: start, end: end, radius: radius }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for Line<T>{
    fn eval(&self, x: T, y: T, z: T) -> T {
        let zero = T::zero();
        let pt = Vec3::new(x, y, z);
        let v1 = pt - self.start;
        let v2 = (self.end - self.start).normalize();
        let t = (v1.dot(v2)).clamp(zero, self.start.distance_to_vec3(self.end));
        let pt_on_line = self.start + (v2 * t);
        pt_on_line.distance_to_vec3(pt) - self.radius 
    }
}