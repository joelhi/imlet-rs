use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::component::{ComponentId, ImplicitOperation};

pub struct Offset<T> {
    inputs: [ComponentId; 1],
    distance: T
}

impl<T: Float + Debug> Offset<T>{
    pub fn new(value: ComponentId, offset_distance: T)->Self{
        Self{
            inputs: [value],
            distance: offset_distance,
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Offset<T> {
    fn eval(&self, inputs: &[T]) -> T {
        inputs[0] - self.distance
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}

pub struct Thickness<T: Float + Debug> {
    inputs: [ComponentId; 1],
    t: T
}

impl<T: Float + Debug> Thickness<T>{
    pub fn new(value: ComponentId, thickness: T)->Self{
        Self{
            inputs: [value],
            t: thickness,
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitOperation<T> for Thickness<T> {
    fn eval(&self, inputs: &[T]) -> T {
        let two = T::from(2.0).unwrap();
        (inputs[0] - self.t / two).max(-(inputs[0] + self.t / two))
    }

    fn get_inputs(&self) -> &[ComponentId] {
        &self.inputs
    }
}