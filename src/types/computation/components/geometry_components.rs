use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{functions::MeshFile, traits::ImplicitFunction},
    geometry::{BoundingBox, Capsule, Line, Plane, Sphere, Torus, Vec3},
};

use super::Component;

/// Public enum to list all the geometry types exposed as model components.
///
/// The geometry components are not fundamentally different from the other functions.
/// Excpect that the in general represent an object of limited size, as opposed to an infinitely periodic surface.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum GeometryComponent {
    /// Represents a component to generate the distance function for a sphere.
    Sphere,
    /// Represents a component to generate the distance function for a torus.
    Torus,
    /// Represents a component to generate the distance function for a torus.
    Plane,
    /// Represents a component to generate the distance function for a plane.
    Box,
    /// Represents a component to generate the distance function for a box.
    Capsule,
    /// Represents a component to generate the distance function for an arbitrary mesh.
    MeshFile,
}

impl GeometryComponent {
    /// Create a default component of the specific type.
    pub fn create_default<T: Float + Send + Sync + 'static + Serialize>(&self) -> Component<T> {
        let default_value = T::from(45.).unwrap();
        let geo: Box<dyn ImplicitFunction<T>> = match self {
            GeometryComponent::Sphere => Box::new(Sphere::new(Vec3::origin(), default_value)),
            GeometryComponent::Torus => Box::new(Torus::new(
                Vec3::origin(),
                default_value,
                T::from(7.5).unwrap(),
            )),
            GeometryComponent::Plane => Box::new(Plane::xy()),
            GeometryComponent::Box => Box::new(BoundingBox::new(
                Vec3::origin(),
                Vec3::new(default_value, default_value, default_value),
            )),
            GeometryComponent::Capsule => Box::new(Capsule::new(
                Line::new(
                    Vec3::new(T::zero(), -default_value, T::zero()),
                    Vec3::new(T::zero(), default_value, T::zero()),
                ),
                T::from(5).unwrap(),
            )),
            GeometryComponent::MeshFile => Box::new(MeshFile::new()),
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
