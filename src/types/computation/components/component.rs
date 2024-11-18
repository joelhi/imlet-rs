use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::{ImplicitFunction, ImplicitOperation};

use super::{Data, DataType, Parameter};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ComponentId(pub usize);

impl From<usize> for ComponentId {
    fn from(value: usize) -> Self {
        ComponentId(value)
    }
}

#[derive(Serialize)]
pub enum Component<T: Float + Send + Sync> {
    Constant(T),
    Function(Box<dyn ImplicitFunction<T>>),
    Operation(Box<dyn ImplicitOperation<T>>),
}

impl<T: Float + Send + Sync> Component<T> {
    pub fn compute(&self, x: T, y: T, z: T, inputs: &[T]) -> T {
        match self {
            Component::Constant(value) => *value,
            Component::Function(function) => function.eval(x, y, z),
            Component::Operation(operation) => operation.eval(inputs),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Component::Constant(_) => "Constant",
            Component::Function(function) => function.function_name(),
            Component::Operation(operation) => operation.operation_name(),
        }
    }

    pub fn get_parameters(&self) -> Vec<(Parameter, Data<T>)> {
        match self {
            Component::Constant(value) => vec![(
                Parameter::new("Value", DataType::Value),
                Data::Value(*value),
            )],
            Component::Function(function) => function
                .parameters()
                .iter()
                .map(|p| {
                    (
                        p.clone(),
                        function.read_parameter(p.name).unwrap_or_else(|| panic!("Parameter {} returned None from function {}, but it should be valid",
                            p.name,
                            function.function_name())),
                    )
                })
                .collect(),
            Component::Operation(operation) => operation
                .parameters()
                .iter()
                .map(|p| {
                    (
                        p.clone(),
                        operation.read_parameter(p.name).unwrap_or_else(|| panic!("Parameter {} returned None from operation {}, but it should be valid",
                            p.name,
                            operation.operation_name())),
                    )
                })
                .collect(),
        }
    }

    pub fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        match self {
            Component::Constant(value) => {
                *value = *data.get_value().expect("This should be a value type.")
            }
            Component::Function(function) => function.set_parameter(parameter_name, data),
            Component::Operation(operation) => operation.set_parameter(parameter_name, data),
        }
    }

    pub fn input_names(&mut self) -> &[&str] {
        match self {
            Component::Constant(_) => &[],
            Component::Function(_) => &[],
            Component::Operation(operation) => operation.inputs(),
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
        T::from(self.values[component_id.0]).expect("Should be able to convert f64 to T")
    }

    pub fn set<T: Float>(&mut self, index: usize, value: T) {
        self.values[index] = value.to_f64().expect("Should be able to convert T to f64");
    }

    pub fn last<T: Float>(&self) -> T {
        T::from(self.values[self.values.len() - 1]).expect("Should be able to convert f64 to T")
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::operations::math::Add,
        geometry::{Sphere, Vec3},
    };

    use super::*;

    #[test]
    fn test_compute_constant() {
        let component = Component::Constant(1.0);

        let value = component.compute(0.0, 0.0, 0.0, &[]);
        assert!((1.0 - value).abs() < 0.001);
    }

    #[test]
    fn test_compute_function() {
        let function = Sphere::new(Vec3::origin(), 1.0);
        let component = Component::Function(Box::new(function));

        assert!((-0.5 - component.compute(0.0, 0.5, 0.0, &[])).abs() < 0.001);
        assert!((0.5 - component.compute(0.0, 1.5, 0.0, &[])).abs() < 0.001);
    }

    #[test]
    fn test_compute_operation() {
        let operation = Add::new();
        let component = Component::Operation(Box::new(operation));

        assert!((2.0 - component.compute(0.0, 0.0, 0.0, &[1.0, 1.0])).abs() < 0.001);
    }
}
