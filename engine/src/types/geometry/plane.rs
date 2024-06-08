use std::fmt::Debug;

use num_traits::Float;

use super::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Plane<T: Float + Debug>{
    origin: Vec3<T>,
    normal: Vec3<T>
}

impl<T> Plane<T> where T: Float + Debug {
    pub fn new(origin: Vec3<T>, normal: Vec3<T>)->Self{
        Plane{
            origin: origin,
            normal: normal.normalize()
        }
    }

    pub fn origin(&self)->Vec3<T>{
        self.origin
    }

    pub fn normal(&self)->Vec3<T>{
        self.normal
    }

    pub fn signed_distance(&self, pt: Vec3<T>)->T{
        let v = pt-self.origin;
        self.normal.dot(v)
    }

    pub fn signed_distance_coord(&self, x: T, y: T, z: T)->T{
        self.normal.dot_vec3(x-self.origin.x, y-self.origin.y, z-self.origin.z)
    }
}