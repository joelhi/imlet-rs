use std::fmt::Debug;

use crate::types::computation::traits::ImplicitFunction;

/// A wrapper for a closure *(x,y,z)->value* which allows a custom function to be computed.
#[derive(Debug, Clone, Copy)]
pub struct CustomFunction<T> {
    pub func: fn(T, T, T) -> T,
}

impl<T> CustomFunction<T> {
    /// Create a new CustomFunction from a closure.
    ///
    /// # Arguments
    ///
    /// * `func` - A function closure, which takes in three values (the x,y and z coordinates) and returns a single value.
    pub fn new(func: fn(T, T, T) -> T) -> Self {
        Self { func }
    }
}

impl<T: Send + Sync> ImplicitFunction<T> for CustomFunction<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        (self.func)(x, y, z)
    }

    fn parameters(&self) -> Vec<crate::types::computation::Parameter> {
        vec![]
    }

    fn set_parameter(&mut self, _: &str, _: crate::types::computation::Data<T>) {
        // Void
    }

    fn read_parameter(&self, _: &str) -> Option<crate::types::computation::Data<T>> {
        None
    }

    fn function_name(&self) -> &'static str {
        "CustomFunction"
    }
}
