use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::ComponentId;

pub trait ImplicitFunction<T: Float + Debug + Send + Sync>: Sync + Send {
    fn eval(&self, x: T, y: T, z: T) -> T;
}

pub trait ImplicitOperation<T: Float + Debug + Send + Sync>: Sync + Send {
    fn eval(&self, inputs: &[T]) -> T;

    fn get_inputs(&self) -> &[ComponentId];
}