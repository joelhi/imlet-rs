use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::traits::ImplicitFunction,
    geometry::{BoundingBox, Plane, Sphere, Torus, Vec3},
};

use super::Component;

/// Public enum to list all the geometry types exposed as model components
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GeometryComponent {
    Sphere,
    Torus,
    Plane,
    Box,
    Capsule,
    MeshFile,
}

impl GeometryComponent {
    /// Create a default component of the specific type
    pub fn create_default<T: Float + Send + Sync + 'static>(&self) -> Component<T> {
        let geo: Box<dyn ImplicitFunction<T>> = match self {
            GeometryComponent::Sphere => {
                Box::new(Sphere::new(Vec3::origin(), T::from(45.).unwrap()))
            }
            GeometryComponent::Torus => Box::new(Torus::new(
                Vec3::origin(),
                T::from(45.).unwrap(),
                T::from(7.5).unwrap(),
            )),
            GeometryComponent::Plane => Box::new(Plane::xy()),
            GeometryComponent::Box => Box::new(BoundingBox::new(
                Vec3::origin(),
                Vec3::new(
                    T::from(45.).unwrap(),
                    T::from(45.).unwrap(),
                    T::from(45.).unwrap(),
                ),
            )),
            GeometryComponent::Capsule => todo!(),
            GeometryComponent::MeshFile => todo!(),
        };

        Component::Function(geo)
    }
}

/// A slice with all the available geometry components
pub const PUBLIC_GEOMETRY_COMPONENTS: &'static [GeometryComponent] = &[
    GeometryComponent::Sphere,
    GeometryComponent::Torus,
    GeometryComponent::Plane,
    GeometryComponent::Capsule,
    GeometryComponent::MeshFile,
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_all_operation_components() {
        let all_operations = PUBLIC_GEOMETRY_COMPONENTS;

        for &operation in all_operations {
            let mut component = operation.create_default::<f32>();
            let params = component.get_parameters();

            for (param, data) in params {
                component.set_parameter(param.name, data);
            }
        }
    }
}
