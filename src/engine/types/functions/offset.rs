use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct Offset<F: ImplicitFunction> {
    pub f: F,
    pub distance: f32,
}

impl<F: ImplicitFunction> Offset<F>{
    pub fn new(f: F, distance: f32)->Self{
        Offset{
            f,
            distance
        }
    }
}

impl<F: ImplicitFunction> ImplicitFunction for Offset<F> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.f.eval(x, y, z) + self.distance
    }
}