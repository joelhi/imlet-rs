use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct Constant {
    pub value: f32,
}

impl Constant {
    pub fn new(value: f32) -> Self {
        Constant { value }
    }
}

impl ImplicitFunction for Constant {
    fn eval(&self, _: f32, _: f32, _: f32) -> f32 {
        self.value
    }
}
