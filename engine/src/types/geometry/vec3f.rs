use std::{fmt::Debug, ops};

use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>
where
    T: Float + Debug,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float + Debug> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn distance_to_vec3(&self, pt: &Vec3<T>) -> T {
        self.distance_to_vec3_squared(&pt).sqrt()
    }

    pub fn distance_to_coord(&self, x: T, y: T, z: T) -> T {
        self.distance_to_coord_squared(x, y, z).sqrt()
    }

    pub fn distance_to_vec3_squared(&self, pt: &Vec3<T>) -> T {
        self.distance_to_coord_squared(pt.x, pt.y, pt.z)
    }

    pub fn distance_to_coord_squared(&self, x: T, y: T, z: T) -> T {
        (self.x - x).powi(2) + (self.y - y).powi(2) + (self.z - z).powi(2)
    }

    pub fn origin() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn x_axis() -> Vec3<T> {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn y_axis() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn z_axis() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    pub fn interpolate(first: &Vec3<T>, second: &Vec3<T>, parameter: T) -> Vec3<T> {
        Self {
            x: first.x + parameter * (second.x - first.x),
            y: first.y + parameter * (second.y - first.y),
            z: first.z + parameter * (second.z - first.z),
        }
    }

    pub fn dot(&self, rhs: Vec3<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn dot_vec3(&self, x: T, y: T, z: T) -> T {
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    pub fn cross(&self, rhs: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn magnitude(&self) -> T {
        self.distance_to_coord(T::zero(), T::zero(), T::zero())
    }

    pub fn normalize(&self) -> Vec3<T> {
        *self * (T::one() / self.magnitude())
    }

    pub fn to_f32(&self) -> Vec3<f32> {
        Vec3 {
            x: self.x.to_f32().expect("Failed to convert to f32"),
            y: self.y.to_f32().expect("Failed to convert to f32"),
            z: self.z.to_f32().expect("Failed to convert to f32"),
        }
    }

    pub fn default_tolerance() -> T{
        T::from(1E-7).expect("Fail")
    }
}

impl<T: Float + Debug> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        {
            Self {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
                z: self.z + _rhs.z,
            }
        }
    }
}

impl<T: Float + Debug> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Float + Debug> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float + Debug> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = T;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.dot(rhs)
    }
}