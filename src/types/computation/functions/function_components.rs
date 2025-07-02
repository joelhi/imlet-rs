use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::types::computation::functions::*;
use crate::types::computation::model::ModelComponent;
use crate::types::computation::traits::ImplicitFunction;
use crate::types::computation::traits::ModelFloat;
use crate::types::geometry::*;

/// Different available function components
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FunctionComponent {
    /// Function to generate a triply periodic Gyroid surface.
    Gyroid,
    /// Function to generate a triply periodic Schwarz-P surface.
    SchwarzP,
    /// Function to generate a triply periodic Neovius surface.
    Neovius,
    /// A remapped domain for the x-coordinate.
    XDomain,
    /// A remapped domain for the y-coordinate.
    YDomain,
    /// A remapped domain for the z-coordinate.
    ZDomain,
    /// Simple function which maps to a coordinate, e.g. *f(x,y,z)->x*
    XYZValue,
    /// Represents a component to generate the distance function for a sphere.
    Sphere,
    /// Represents a component to generate the distance function for a torus.
    Torus,
    /// Represents a component to generate the distance function for a plane.
    Plane,
    /// Represents a component to generate the distance function for a box.
    BoundingBox,
    /// Represents a component to generate the distance function for a capsule.
    Capsule,
    /// Represents a component to generate the distance function for an arbitrary mesh.
    MeshFile,
}

impl FunctionComponent {
    /// Create an instance of the component with default values.
    ///
    /// Used when creating components from a UI or other interface.
    pub fn create_default<T: ModelFloat + 'static>(&self) -> ModelComponent<T> {
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
            FunctionComponent::XYZValue => Box::new(XYZValue::new(CoordinateValue::X)),
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
            FunctionComponent::XDomain => Box::new(XDomain::natural()),
            FunctionComponent::YDomain => Box::new(YDomain::natural()),
            FunctionComponent::ZDomain => Box::new(ZDomain::natural()),
        };

        ModelComponent::Function(func)
    }
}

impl FromStr for FunctionComponent {
    type Err = ();

    fn from_str(input: &str) -> Result<FunctionComponent, Self::Err> {
        match input {
            "Gyroid" => Ok(FunctionComponent::Gyroid),
            "SchwarzP" => Ok(FunctionComponent::SchwarzP),
            "Neovius" => Ok(FunctionComponent::Neovius),
            "XDomain" => Ok(FunctionComponent::XDomain),
            "YDomain" => Ok(FunctionComponent::YDomain),
            "ZDomain" => Ok(FunctionComponent::ZDomain),
            "XYZValue" => Ok(FunctionComponent::XYZValue),
            "Sphere" => Ok(FunctionComponent::Sphere),
            "Torus" => Ok(FunctionComponent::Torus),
            "Plane" => Ok(FunctionComponent::Plane),
            "BoundingBox" => Ok(FunctionComponent::BoundingBox),
            "Capsule" => Ok(FunctionComponent::Capsule),
            "MeshFile" => Ok(FunctionComponent::MeshFile),
            _ => Err(()),
        }
    }
}

/// List of the different function components
pub const FUNCTION_COMPONENTS: &[FunctionComponent] = &[
    FunctionComponent::Gyroid,
    FunctionComponent::SchwarzP,
    FunctionComponent::Neovius,
    FunctionComponent::XYZValue,
    FunctionComponent::XDomain,
    FunctionComponent::YDomain,
    FunctionComponent::ZDomain,
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
    fn test_all_function_components_params() {
        let all_functions = FUNCTION_COMPONENTS;

        for &function in all_functions {
            let mut component = function.create_default::<f32>();
            let params = component.read_parameters();

            for (param, data) in params {
                component.set_parameter(param.name, data);
            }
        }
    }
}
