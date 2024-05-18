const MAX_INPUTS: usize = 8;

#[derive(Debug, Copy, Clone)]
pub struct ComponentId(usize);

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

pub enum Component {
    Constant(f32),
    Function(Box<dyn ImplicitFunction>),
    Operation(Box<dyn ImplicitOperation>),
}

impl Component {
    pub fn compute(&self, x: f32, y: f32, z: f32, values: &[f32]) -> f32 {
        match self {
            Component::Constant(value) => *value,
            Component::Function(function) => function.eval(x, y, z),
            Component::Operation(operation) => {
                operation.eval(&Self::get_input_data(&operation.get_inputs(), values))
            }
        }
    }

    pub fn get_input_data(inputs: &[ComponentId], values: &[f32]) -> [f32; MAX_INPUTS] {
        let mut result = [0.0; MAX_INPUTS];
        for (i, &id) in inputs.iter().enumerate() {
            result[i] = values[id.0];
        }
        result
    }
}

pub trait ImplicitFunction: Sync + Send {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32;
}

pub trait ImplicitOperation: Sync + Send {
    fn eval(&self, inputs: &[f32]) -> f32;

    fn get_inputs(&self) -> &[ComponentId];
}

#[cfg(test)]
mod tests {
    use crate::engine::types::{
        computation::{distance_functions::Sphere, operations::arithmetic::Add},
        geometry::Vec3f,
    };

    use super::*;

    #[test]
    fn test_compute_constant() {
        let component = Component::Constant(1.0);

        let values = [0.0; 0];
        assert!((1.0 - component.compute(0.0, 0.0, 0.0, &values)).abs() < 0.001);
    }

    #[test]
    fn test_compute_function() {
        let function = Sphere::new(Vec3f::origin(), 1.0);
        let component = Component::Function(Box::new(function));

        let values = [0.0; 0];
        assert!((-0.5 - component.compute(0.0, 0.5, 0.0, &values)).abs() < 0.001);
        assert!((0.5 - component.compute(0.0, 1.5, 0.0, &values)).abs() < 0.001);
    }

    #[test]
    fn test_compute_operation() {
        let operation = Add::new(0.into(), 1.into());
        let component = Component::Operation(Box::new(operation));

        let values = [1.0; 2];
        assert!((2.0 - component.compute(0.0, 0.0, 0.0, &values)).abs() < 0.001);
    }
}
