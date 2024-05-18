use super::Vec3f;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl BoundingBox {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        BoundingBox { min, max }
    }

    pub fn get_dimensions(&self) -> (f32, f32, f32) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn is_inside(&self, pt: Vec3f) -> bool {
        pt.x > self.min.x
            && pt.y > self.min.y
            && pt.z > self.min.z
            && pt.x < self.max.x
            && pt.y < self.max.y
            && pt.z < self.max.z
    }

    pub fn is_coord_inside(&self, x: f32, y: f32, z: f32) -> bool {
        x > self.min.x
            && y > self.min.y
            && z > self.min.z
            && x < self.max.x
            && y < self.max.y
            && z < self.max.z
    }
}
