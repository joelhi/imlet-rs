use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {

    pub fn new(x: f32, y: f32, z: f32)->Self{
        Vec3f {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn distance_to_vec3f(&self, pt: Vec3f) -> f32 {
        self.distance_to_vec3f_squared(pt).sqrt()
    }

    pub fn distance_to_coord(&self, x: f32, y: f32, z: f32) -> f32 {
        self.distance_to_coord_squared(x, y, z).sqrt()
    }

    pub fn distance_to_vec3f_squared(&self, pt: Vec3f) -> f32 {
        (self.x - pt.x).powi(2) + (self.y - pt.y).powi(2) + (self.z - pt.z).powi(2)
    }

    pub fn distance_to_coord_squared(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.x - x).powi(2) + (self.y - y).powi(2) + (self.z - z).powi(2)
    }

    pub fn origin() -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x_axis() -> Vec3f {
        Vec3f {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn y_axis() -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn z_axis() -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn interpolate(first: &Vec3f, second: &Vec3f, parameter: f32) -> Vec3f {
        Vec3f {
            x: first.x + parameter * (second.x - first.x),
            y: first.y + parameter * (second.y - first.y),
            z: first.z + parameter * (second.z - first.z),
        }
    }

    pub fn dot(&self, rhs: Vec3f) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn dot_vec3f(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    pub fn cross(&self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn magnitude(&self)->f32{
        self.distance_to_coord(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self)->Vec3f{
        *self/self.magnitude()
    }
}

impl ops::Add<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, _rhs: Vec3f) -> Vec3f {
        {
            Vec3f {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
                z: self.z + _rhs.z,
            }
        }
    }
}

impl ops::Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, _rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        Vec3f {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<Vec3f> for Vec3f {
    type Output = f32;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        self.dot(rhs)
    }
}

impl ops::Div<f32> for Vec3f{
    type Output = Vec3f;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3f {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
