use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::traits::implicit_functions::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct CustomFunction<T: Float + Debug> {
    pub func: fn(T, T, T) -> T,
}

impl<T: Float + Debug> CustomFunction<T> {
    pub fn new(func: fn(T, T, T) -> T) -> Self {
        Self { func }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for CustomFunction<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        (self.func)(x, y, z)
    }
}
