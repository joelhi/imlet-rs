use crate::types::computation::components::{
    function_components::FunctionComponent, operation_components::OperationComponent,
};

/// List of the components exposed in the app under the functions.
pub const PUBLIC_FUNCTION_COMPONENTS: &'static [FunctionComponent] = &[
    FunctionComponent::Gyroid,
    FunctionComponent::SchwarzP,
    FunctionComponent::Neovius,
    FunctionComponent::XYZValue,
];

/// List of the components exposed in the app under the geometry category.
pub const PUBLIC_GEOMETRY_COMPONENTS: &'static [FunctionComponent] = &[
    FunctionComponent::Sphere,
    FunctionComponent::Torus,
    FunctionComponent::Plane,
    FunctionComponent::Capsule,
    FunctionComponent::MeshFile,
];

/// List of the components exposed in the app under the operations category
pub const PUBLIC_OPERATION_COMPONENTS: &'static [OperationComponent] = &[
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
