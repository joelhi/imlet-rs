use num_traits::Float;
use serde::Deserialize;
use serde::Serialize;

use crate::types::computation::functions::CoordinateValue;
use crate::types::computation::functions::Gyroid;
use crate::types::computation::functions::Neovius;
use crate::types::computation::functions::SchwarzP;
use crate::types::computation::functions::XYZCoordinate;
use crate::types::computation::traits::ImplicitFunction;
use crate::utils::math_helper::Pi;

use super::Component;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FunctionComponent {
    Gyroid,
    SchwarzP,
    Neovius,
    XYZValue,
}

impl FunctionComponent {
    /// Create an instance of the component with default values.
    ///
    /// Used when creating components from a UI or other interface.
    pub fn create_default<T: Float + Pi + Send + Sync + 'static + Serialize>(&self) -> Component<T> {
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
}
