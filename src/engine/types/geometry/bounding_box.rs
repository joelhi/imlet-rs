use super::Vec3f;

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
}
