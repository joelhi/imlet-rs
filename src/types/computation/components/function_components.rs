use num_traits::Float;
use serde::Deserialize;
use serde::Serialize;

use crate::types::computation::functions::CoordinateValue;
use crate::types::computation::functions::Gyroid;
use crate::types::computation::functions::MeshFile;
use crate::types::computation::functions::Neovius;
use crate::types::computation::functions::SchwarzP;
use crate::types::computation::functions::XYZCoordinate;
use crate::types::computation::traits::ImplicitFunction;
use crate::types::geometry::*;
use crate::utils::math_helper::Pi;

use super::Component;

/// Different available function components
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FunctionComponent {
    /// Function to generate a triply periodic Gyroid surface.
    Gyroid,
    /// Function to generate a triply periodic Schwarz-P surface.
    SchwarzP,
    /// Function to generate a triply periodic Neovius surface.
    Neovius,
    /// Simple function which maps to a coordinate, e.g. *f(x,y,z)->x*
    XYZValue,
    /// Represents a component to generate the distance function for a sphere.
    Sphere,
    /// Represents a component to generate the distance function for a torus.
    Torus,
    /// Represents a component to generate the distance function for a torus.
    Plane,
    /// Represents a component to generate the distance function for a plane.
    BoundingBox,
    /// Represents a component to generate the distance function for a box.
    Capsule,
    /// Represents a component to generate the distance function for an arbitrary mesh.
    MeshFile,
}

impl FunctionComponent {
    /// Create an instance of the component with default values.
    ///
    /// Used when creating components from a UI or other interface.
    pub fn create_default<T: Float + Pi + Send + Sync + 'static + Serialize>(
        &self,
    ) -> Component<T> {
        let default_value = T::from(45.).unwrap();
        let func: Box<dyn ImplicitFunction<T>> = match self {
            FunctionComponent::Gyroid => {
                Box::new(Gyroid::with_equal_spacing(T::from(15).unwrap(), false))
            }
            FunctionComponent::SchwarzP => {
                Box::new(SchwarzP::with_equal_spacing(T::from(15).unwrap(), false))
            }
            FunctionComponent::Neovius => {
                Box::new(Neovius::with_equal_spacing(T::from(15).unwrap(), false))
            }
            FunctionComponent::XYZValue => Box::new(XYZCoordinate::new(CoordinateValue::X)),
            FunctionComponent::Sphere => Box::new(Sphere::new(Vec3::origin(), default_value)),
            FunctionComponent::Torus => Box::new(Torus::new(
                Vec3::origin(),
                default_value,
                T::from(7.5).unwrap(),
            )),
            FunctionComponent::Plane => Box::new(Plane::xy()),
            FunctionComponent::BoundingBox => Box::new(BoundingBox::new(
                Vec3::origin(),
                Vec3::new(default_value, default_value, default_value),
            )),
            FunctionComponent::Capsule => Box::new(Capsule::new(
                Line::new(
                    Vec3::new(T::zero(), -default_value, T::zero()),
                    Vec3::new(T::zero(), default_value, T::zero()),
                ),
                T::from(5).unwrap(),
            )),
            FunctionComponent::MeshFile => Box::new(MeshFile::new()),
        };

        Component::Function(func)
    }
}

pub const PUBLIC_FUNCTION_COMPONENTS: &'static [FunctionComponent] = &[
    FunctionComponent::Gyroid,
    FunctionComponent::SchwarzP,
    FunctionComponent::Neovius,
    FunctionComponent::XYZValue,
];

/// A slice with all the available geometry components
pub const PUBLIC_GEOMETRY_COMPONENTS: &'static [FunctionComponent] = &[
    FunctionComponent::Sphere,
    FunctionComponent::Torus,
    FunctionComponent::Plane,
    FunctionComponent::Capsule,
    FunctionComponent::MeshFile,
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_all_function_components() {
        let all_functions = PUBLIC_FUNCTION_COMPONENTS;

        for &function in all_functions {
            let mut component = function.create_default::<f32>();
            let params = component.get_parameters();

            for (param, data) in params {
                component.set_parameter(param.name, data);
            }
        }
    }

    #[test]
    fn test_all_geometry_components() {
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
