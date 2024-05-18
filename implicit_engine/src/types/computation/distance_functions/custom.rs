use crate::engine::types::computation::component::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct CustomFunction {
    pub func: fn(f32, f32, f32) -> f32,
}

impl CustomFunction {
    pub fn new(func: fn(f32, f32, f32) -> f32) -> Self {
        CustomFunction { func }
    }
}

impl ImplicitFunction for CustomFunction {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        (self.func)(x, y, z)
    }
}
