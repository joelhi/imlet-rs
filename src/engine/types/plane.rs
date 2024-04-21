use super::XYZ;

#[derive(Debug, Clone, Copy)]
pub struct Plane{
    origin: XYZ,
    normal: XYZ
}

impl Plane{
    pub fn new(origin: XYZ, normal: XYZ)->Self{
        Plane{
            origin: origin,
            normal: normal.normalize()
        }
    }

    pub fn origin(&self)->XYZ{
        self.origin
    }

    pub fn normal(&self)->XYZ{
        self.normal
    }

    pub fn signed_distance(&self, pt: XYZ)->f32{
        let v = pt-self.origin;
        self.normal.dot(v)
    }
}