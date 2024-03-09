use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn distance_to(&self, pt: XYZ) -> f32 {
        ((self.x - pt.x).powi(2) + (self.y - pt.y).powi(2) + (self.z - pt.z).powi(2)).sqrt()
    }

    pub fn get_origin() -> XYZ {
        XYZ { x: 0.0, y: 0.0, z: 0.0 }
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

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: XYZ,
    pub p2: XYZ,
    pub p3: XYZ,
}
