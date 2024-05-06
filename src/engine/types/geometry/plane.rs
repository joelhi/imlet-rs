use super::Vec3f;

#[derive(Debug, Clone, Copy)]
pub struct Plane{
    origin: Vec3f,
    normal: Vec3f
}

impl Plane{
    pub fn new(origin: Vec3f, normal: Vec3f)->Self{
        Plane{
            origin: origin,
            normal: normal.normalize()
        }
    }

    pub fn origin(&self)->Vec3f{
        self.origin
    }

    pub fn normal(&self)->Vec3f{
        self.normal
    }

    pub fn signed_distance(&self, pt: Vec3f)->f32{
        let v = pt-self.origin;
        self.normal.dot(v)
    }
}