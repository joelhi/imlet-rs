use crate::types::computation::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct ZDomain {
    min: f32,
    max: f32
 }

impl ZDomain {
    pub fn remapped(min: f32, max: f32) -> Self {
        ZDomain {
            min,
            max
        }
    }

    pub fn natural()->Self{
        ZDomain{
            min: 0.0,
            max: 1.0,
        }
    }
}

impl ImplicitFunction for ZDomain {
    fn eval(&self, _: f32, _: f32, z: f32) -> f32 {
        normalize(z, self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct YDomain {
    min: f32,
    max: f32
 }

impl YDomain {
    pub fn remapped(min: f32, max: f32) -> Self {
        YDomain {
            min,
            max
        }
    }

    pub fn natural()->Self{
        YDomain{
            min: 0.0,
            max: 1.0,
        }
    }
}

impl ImplicitFunction for YDomain {
    fn eval(&self, _: f32, y: f32, _: f32) -> f32 {
        normalize(y, self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XDomain {
    min: f32,
    max: f32
 }

impl XDomain {
    pub fn remapped(min: f32, max: f32) -> Self {
        XDomain {
            min,
            max
        }
    }

    pub fn natural()->Self{
        XDomain{
            min: 0.0,
            max: 1.0,
        }
    }
}

impl ImplicitFunction for XDomain {
    fn eval(&self, x: f32, _: f32, _: f32) -> f32 {
        normalize(x, self.min, self.max)
    }
}

fn normalize(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}