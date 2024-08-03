use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::traits::implicit_functions::{ImplicitFunction, ImplicitOperation};

const MAX_INPUTS: usize = 8;
const MAX_TOTAL_COMPONENTS: usize = 512;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ComponentId(pub usize);

impl ComponentId {
    pub fn value(&self) -> usize {
        self.0
    }
}

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
    pub fn compute(&self, x: T, y: T, z: T, values: &mut ComponentValues, index: usize) {
        values.set(
            index,
            match self {
                Component::Constant(value) => *value,
                Component::Function(function) => function.eval(x, y, z),
                Component::Operation(operation) => {
                    operation.eval(&Self::get_input_data(&operation.get_inputs(), values))
                }
            },
        )
    }

    pub fn get_input_data(inputs: &[ComponentId], values: &ComponentValues) -> [T; MAX_INPUTS] {
        let mut result = [T::zero(); MAX_INPUTS];
        for (i, &id) in inputs.iter().enumerate() {
            result[i] = values.get(id);
        }
        result
    }
}

pub struct ComponentValues {
    values: [f64; MAX_TOTAL_COMPONENTS],
}

impl ComponentValues {
    pub fn new() -> Self {
        ComponentValues {
            values: [0.0; MAX_TOTAL_COMPONENTS],
        }
    }

    pub fn get<T: Float>(&self, component_id: ComponentId) -> T {
        T::from(self.values[component_id.0]).expect("Failed to convert component output to T")
    }

    pub fn set<T: Float + Debug + Send + Sync>(&mut self, index: usize, value: T) {
        self.values[index] = value.to_f64().expect("Failed to convert value to f64");
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        computation::{distance_functions::Sphere, operations::arithmetic::Add},
        geometry::Vec3,
    };

    use super::*;

    #[test]
    fn test_compute_constant() {
        let component = Component::Constant(1.0);

        let mut component_values = ComponentValues::new();
        component.compute(0.0, 0.0, 0.0, &mut component_values, 0);
        assert!((1.0 - component_values.get::<f64>(ComponentId(0))).abs() < 0.001);
    }

    #[test]
    fn test_compute_function() {
        let function = Sphere::new(Vec3::origin(), 1.0);
        let component = Component::Function(Box::new(function));

        let mut component_values = ComponentValues::new();
        component.compute(0.0, 0.5, 0.0, &mut component_values, 0);
        component.compute(0.0, 1.5, 0.0, &mut component_values, 1);
        assert!((-0.5 - component_values.get::<f64>(ComponentId(0))).abs() < 0.001);
        assert!((0.5 - component_values.get::<f64>(ComponentId(1))).abs() < 0.001);
    }

    #[test]
    fn test_compute_operation() {
        let operation = Add::new(0.into(), 1.into());
        let component = Component::Operation(Box::new(operation));

        let mut component_values = ComponentValues::new();
        component_values.set(0, 1.0);
        component_values.set(1, 1.0);
        component.compute(0.0, 0.0, 0.0, &mut component_values, 2);
        assert!((2.0 - component_values.get::<f64>(ComponentId(2))).abs() < 0.001);
    }
}
