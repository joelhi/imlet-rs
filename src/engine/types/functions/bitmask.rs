use super::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct BitMask<T: ImplicitFunction> {
    pub function: T,
    pub cut_off: f32,
}

impl<T: ImplicitFunction> BitMask<T>{
    pub fn new(function: T, cut_off: f32)->Self{
        BitMask{
            function,
            cut_off,
        }
    }
}

impl<T: ImplicitFunction> ImplicitFunction for BitMask<T> {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        match self.function.eval(x, y, z) {
            val if val > self.cut_off => 0.0,
            _ => 1.0,
        }
    }
}