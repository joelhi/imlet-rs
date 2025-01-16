use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    types::computation::traits::{ImplicitFunction, ImplicitOperation},
    utils::math_helper::Pi,
};

use super::{Data, DataType, Parameter};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct ComponentId(pub usize);

impl From<usize> for ComponentId {
    fn from(value: usize) -> Self {
        ComponentId(value)
    }
}

/// Enum which represents the different types of component that make up a computation model.
///
/// The enum represents the high-level types, for which different implementations can be provided.
///
/// The current component types are:
///
/// * `Constant` - Representing a constant value across the entire model domain.
/// * `Function` - Represents a function in 3d space `f(x,y,z)`. This takes no inputs from other components, and is variable across the domain.
/// * `Operation` - Represents an operation on some values in the model. The operation does not depend on the evaluation coordinate directly, but instead operates on the output of other components.
#[derive(Serialize, Deserialize)]
pub enum ModelComponent<T: Float + Send + Sync + Serialize + 'static + Pi> {
    Constant(T),
    Function(Box<dyn ImplicitFunction<T>>),
    Operation(Box<dyn ImplicitOperation<T>>),
}

impl<T: Float + Send + Sync + Serialize + Pi> ModelComponent<T> {
    /// Evaluate the output of the comppnent
    ///
    /// # Arguments
    ///
    /// * `x` - The current x coordinate. Used when the type is [`ModelComponent::Function`].
    /// * `y` - The current y coordinate. Used when the type is [`ModelComponent::Function`].
    /// * `z` - The current z coordinate. Used when the type is [`ModelComponent::Function`].
    /// * `inputs` - The outputs of other components which feed the inputs of this one. Used when the type is [`ModelComponent::Operation`].
    pub fn compute(&self, x: T, y: T, z: T, inputs: &[T]) -> T {
        match self {
            ModelComponent::Constant(value) => *value,
            ModelComponent::Function(function) => function.eval(x, y, z),
            ModelComponent::Operation(operation) => operation.eval(inputs),
        }
    }

    /// Returns the type of the function or operation inside the component.
    pub fn type_name(&self) -> &'static str {
        match self {
            ModelComponent::Constant(_) => "Constant",
            ModelComponent::Function(function) => function.name(),
            ModelComponent::Operation(operation) => operation.name(),
        }
    }

    pub fn get_parameters(&self) -> Vec<(Parameter, Data<T>)> {
        match self {
            ModelComponent::Constant(value) => vec![(
                Parameter::new("Value", DataType::Value),
                Data::Value(*value),
            )],
            ModelComponent::Function(function) => function
                .parameters()
                .iter()
                .map(|p| {
                    (
                        p.clone(),
                        function.read_parameter(p.name).unwrap_or_else(|| panic!("Parameter {} returned None from function {}, but it should be valid",
                            p.name,
                            function.name())),
                    )
                })
                .collect(),
            ModelComponent::Operation(operation) => operation
                .parameters()
                .iter()
                .map(|p| {
                    (
                        p.clone(),
                        operation.read_parameter(p.name).unwrap_or_else(|| panic!("Parameter {} returned None from operation {}, but it should be valid",
                            p.name,
                            operation.name())),
                    )
                })
                .collect(),
        }
    }

    /// Set the value of a parameter for the component.
    pub fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        match self {
            ModelComponent::Constant(value) => {
                *value = *data.get_value().expect("This should be a value type.")
            }
            ModelComponent::Function(function) => function.set_parameter(parameter_name, data),
            ModelComponent::Operation(operation) => operation.set_parameter(parameter_name, data),
        }
    }

    /// Get the tags of the inputs of this component.
    pub fn input_names(&mut self) -> &[&str] {
        match self {
            ModelComponent::Constant(_) => &[],
            ModelComponent::Function(_) => &[],
            ModelComponent::Operation(operation) => operation.inputs(),
        }
    }
}

/// Struct to handle storing of intermediate outputs of components during computation.
pub(crate) struct ComponentValues {
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
        let component = ModelComponent::Constant(1.0);

        let value = component.compute(0.0, 0.0, 0.0, &[]);
        assert!((1.0 - value).abs() < 0.001);
    }

    #[test]
    fn test_compute_function() {
        let function = Sphere::new(Vec3::origin(), 1.0);
        let component = ModelComponent::Function(Box::new(function));

        assert!((-0.5 - component.compute(0.0, 0.5, 0.0, &[])).abs() < f64::epsilon());
        assert!((0.5 - component.compute(0.0, 1.5, 0.0, &[])).abs() < f64::epsilon());
    }

    #[test]
    fn test_compute_operation() {
        let operation = Add::new();
        let component = ModelComponent::Operation(Box::new(operation));

        assert!((2.0 - component.compute(0.0, 0.0, 0.0, &[1.0, 1.0])).abs() < f64::epsilon());
    }
}
