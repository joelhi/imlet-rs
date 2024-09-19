use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::traits::implicit_functions::{ImplicitFunction, ImplicitOperation};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ComponentId(pub usize);

impl From<usize> for ComponentId {
    fn from(value: usize) -> Self {
        ComponentId(value)
    }
}

pub enum Component<T: Float + Debug> {
    Constant(T),
    Function(Box<dyn ImplicitFunction<T>>),
    Operation(Box<dyn ImplicitOperation<T>>),
}

impl<T: Float + Debug + Send + Sync> Component<T> {
    pub fn compute(&self, x: T, y: T, z: T, inputs: &[T]) -> T {
        match self {
            Component::Constant(value) => *value,
            Component::Function(function) => function.eval(x, y, z),
            Component::Operation(operation) => operation.eval(inputs),
        }
    }
}

pub struct ComponentValues {
    values: Vec<f64>,
}

impl ComponentValues {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn resize(&mut self, size: usize) {
        self.values.resize(size, 0.0)
    }

    pub fn get<T: Float>(&self, component_id: ComponentId) -> T {
        T::from(self.values[component_id.0]).expect("Failed to convert component output to T")
    }

    pub fn set<T: Float + Debug + Send + Sync>(&mut self, index: usize, value: T) {
        self.values[index] = value.to_f64().expect("Failed to convert value to f64");
    }

    pub fn last<T: Float>(&self) -> T {
        T::from(self.values[self.values.len() - 1])
            .expect("Failed to convert component output to T")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::{distance_functions::Sphere, operations::math::Add},
        geometry::Vec3,
    };

    use super::*;

    #[test]
    fn test_compute_constant() {
        let component = Component::Constant(1.0);

        let value = component.compute(0.0, 0.0, 0.0, &vec![]);
        assert!((1.0 - value).abs() < 0.001);
    }

    #[test]
    fn test_compute_function() {
        let function = Sphere::new(Vec3::origin(), 1.0);
        let component = Component::Function(Box::new(function));

        assert!((-0.5 - component.compute(0.0, 0.5, 0.0, &vec![])).abs() < 0.001);
        assert!((0.5 - component.compute(0.0, 1.5, 0.0, &vec![])).abs() < 0.001);
    }

    #[test]
    fn test_compute_operation() {
        let operation = Add::new();
        let component = Component::Operation(Box::new(operation));

        assert!((2.0 - component.compute(0.0, 0.0, 0.0, &vec![1.0, 1.0])).abs() < 0.001);
    }
}
