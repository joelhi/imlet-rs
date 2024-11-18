use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    types::computation::{
        operations::{
            math::{Add, Divide, LinearInterpolation, Multiply, Subtract},
            shape::{BooleanDifference, BooleanIntersection, BooleanUnion, Offset, Thickness},
        },
        traits::ImplicitOperation,
    },
    utils::math_helper::Pi,
};

use super::Component;

/// Enum listing valid operation components.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OperationComponent {
    Add,
    Subtract,
    Multiply,
    Divide,
    LinearInterpolation,
    BooleanUnion,
    BooleanDifference,
    BooleanIntersection,
    Offset,
    Thickness,
}

impl OperationComponent {
    /// Create an instance of the component with default values.
    ///
    /// Used when creating components from a UI or other interface.
    pub fn create_default<T: Float + Send + Sync + 'static + Serialize + Pi>(
        &self,
    ) -> Component<T> {
        let op: Box<dyn ImplicitOperation<T>> = match self {
            // Maths
            OperationComponent::Add => Box::new(Add::new()),
            OperationComponent::Subtract => Box::new(Subtract::new()),
            OperationComponent::Multiply => Box::new(Multiply::new()),
            OperationComponent::Divide => Box::new(Divide::new()),
            OperationComponent::LinearInterpolation => Box::new(LinearInterpolation::new()),
            // Shape
            OperationComponent::BooleanUnion => Box::new(BooleanUnion::new()),
            OperationComponent::BooleanIntersection => Box::new(BooleanIntersection::new()),
            OperationComponent::BooleanDifference => Box::new(BooleanDifference::new()),
            OperationComponent::Offset => Box::new(Offset::new(T::zero())),
            OperationComponent::Thickness => Box::new(Thickness::new(T::one())),
        };

        Component::Operation(op)
    }
}

/// List of available operation
pub const OPERATION_COMPONENTS: &'static [OperationComponent] = &[
    // Maths
    OperationComponent::Add,
    OperationComponent::Subtract,
    OperationComponent::Multiply,
    OperationComponent::Divide,
    OperationComponent::LinearInterpolation,
    // Shape
    OperationComponent::BooleanUnion,
    OperationComponent::BooleanIntersection,
    OperationComponent::BooleanDifference,
    OperationComponent::Offset,
    OperationComponent::Thickness,
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_all_operation_components_params() {
        let all_operations = OPERATION_COMPONENTS;

        for &operation in all_operations {
            let mut component = operation.create_default::<f32>();
            let params = component.get_parameters();

            for (param, data) in params {
                component.set_parameter(param.name, data);
            }
        }
    }
}
