use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {

    pub fn new(x: f32, y: f32, z: f32)->Self{
        XYZ {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn distance_to_xyz(&self, pt: XYZ) -> f32 {
        self.distance_to_xyz_squared(pt).sqrt()
    }

    pub fn distance_to_coord(&self, x: f32, y: f32, z: f32) -> f32 {
        self.distance_to_coord_squared(x, y, z).sqrt()
    }

    pub fn distance_to_xyz_squared(&self, pt: XYZ) -> f32 {
        (self.x - pt.x).powi(2) + (self.y - pt.y).powi(2) + (self.z - pt.z).powi(2)
    }

    pub fn distance_to_coord_squared(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.x - x).powi(2) + (self.y - y).powi(2) + (self.z - z).powi(2)
    }

    pub fn origin() -> XYZ {
        XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x_axis() -> XYZ {
        XYZ {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn y_axis() -> XYZ {
        XYZ {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn z_axis() -> XYZ {
        XYZ {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn interpolate(first: &XYZ, second: &XYZ, parameter: f32) -> XYZ {
        XYZ {
            x: first.x + parameter * (second.x - first.x),
            y: first.y + parameter * (second.y - first.y),
            z: first.z + parameter * (second.z - first.z),
        }
    }

    pub fn dot(&self, rhs: XYZ) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn dot_xyz(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    pub fn cross(&self, rhs: XYZ) -> XYZ {
        XYZ {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn magnitude(&self)->f32{
        self.distance_to_coord(0.0, 0.0, 0.0)
    }

    pub fn normalize(&self)->XYZ{
        *self/self.magnitude()
    }
}

impl ops::Add<XYZ> for XYZ {
    type Output = XYZ;
    fn add(self, _rhs: XYZ) -> XYZ {
        {
            XYZ {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
                z: self.z + _rhs.z,
            }
        }
    }
}

impl ops::Sub<XYZ> for XYZ {
    type Output = XYZ;
    fn sub(self, _rhs: XYZ) -> XYZ {
        XYZ {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<f32> for XYZ {
    type Output = XYZ;
    fn mul(self, rhs: f32) -> Self::Output {
        XYZ {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<XYZ> for f32 {
    type Output = XYZ;
    fn mul(self, rhs: XYZ) -> Self::Output {
        XYZ {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<XYZ> for XYZ {
    type Output = f32;
    fn mul(self, rhs: XYZ) -> Self::Output {
        self.dot(rhs)
    }
}

impl ops::Div<f32> for XYZ{
    type Output = XYZ;
    fn div(self, rhs: f32) -> Self::Output {
        XYZ {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
