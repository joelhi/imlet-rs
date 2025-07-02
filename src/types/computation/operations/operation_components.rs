#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::ModelComponent,
    operations::{
        math::{
            Add, Divide, LinearInterpolation, Multiply, Remap, Subtract,
            VariableLinearInterpolation,
        },
        shape::{BooleanDifference, BooleanIntersection, BooleanUnion, Offset, Thickness},
    },
    traits::{ImplicitOperation, ModelFloat},
};

/// Enum listing valid operation components.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum OperationComponent {
    Add,
    Subtract,
    Multiply,
    Divide,
    LinearInterpolation,
    VariableLinearInterpolation,
    Remap,
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
    pub fn create_default<T: ModelFloat + 'static>(&self) -> ModelComponent<T> {
        let op: Box<dyn ImplicitOperation<T>> = match self {
            // Maths
            OperationComponent::Add => Box::new(Add::new()),
            OperationComponent::Subtract => Box::new(Subtract::new()),
            OperationComponent::Multiply => Box::new(Multiply::new()),
            OperationComponent::Divide => Box::new(Divide::new()),
            OperationComponent::LinearInterpolation => Box::new(LinearInterpolation::new()),
            OperationComponent::VariableLinearInterpolation => {
                Box::new(VariableLinearInterpolation::new())
            }
            OperationComponent::Remap => Box::new(Remap::new()),
            // Shape
            OperationComponent::BooleanUnion => Box::new(BooleanUnion::new()),
            OperationComponent::BooleanIntersection => Box::new(BooleanIntersection::new()),
            OperationComponent::BooleanDifference => Box::new(BooleanDifference::new()),
            OperationComponent::Offset => Box::new(Offset::new(T::zero())),
            OperationComponent::Thickness => Box::new(Thickness::new(T::one())),
        };

        ModelComponent::Operation(op)
    }
}

/// List of available operation
pub const OPERATION_COMPONENTS: &[OperationComponent] = &[
    // Maths
    OperationComponent::Add,
    OperationComponent::Subtract,
    OperationComponent::Multiply,
    OperationComponent::Divide,
    OperationComponent::LinearInterpolation,
    OperationComponent::VariableLinearInterpolation,
    OperationComponent::Remap,
    OperationComponent::VariableLinearInterpolation,
    OperationComponent::Remap,
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
            let params = component.read_parameters();

            for (param, data) in params {
                component.set_parameter(param.name, data);
            }
        }
    }
}
