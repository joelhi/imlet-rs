use crate::types::{
    computation::ImplicitFunction,
    geometry::{BoundingBox, Vec3f},
};

#[derive(Debug, Clone, Copy)]
pub struct OrthoBox {
    pub bounds: BoundingBox,
}

impl OrthoBox {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        OrthoBox {
            bounds: BoundingBox::new(min, max),
        }
    }

    pub fn from_size(origin: Vec3f, size: f32) -> Self {
        OrthoBox {
            bounds: BoundingBox::new(
                origin,
                Vec3f::new(origin.x + size, origin.y + size, origin.z + size),
            ),
        }
    }
}

impl ImplicitFunction for OrthoBox {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let pt = Vec3f::new(x, y, z);

        let diff1 = self.bounds.max - pt;
        let diff2 = self.bounds.min - pt;

        let dist = diff1.x.abs().min(
            diff1.y.abs().min(
                diff1
                    .z
                    .abs()
                    .min(diff2.x.abs().min(diff2.y.abs().min(diff2.z.abs()))),
            ),
        );

        if self.bounds.is_coord_inside(x, y, z){
            -dist
        }else{
            dist
        }
    }
}
