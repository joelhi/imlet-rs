use std::{marker::PhantomData, str::FromStr};

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        computation::{
            functions::*,
            operations::{math::*, shape::*},
            traits::{ImplicitFunction, ImplicitOperation},
        },
        geometry::*,
    },
    utils::math_helper::Pi,
};

use super::function_components::FunctionComponent;

impl<T: Float + Send + Sync> serde::Serialize for dyn ImplicitFunction<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut ser = serializer.serialize_map(Some(1))?;
        let type_info = self.function_name();
        ser.serialize_entry(type_info, &Wrap(self))?;
        ser.end()
    }
}

impl<T: Float + Send + Sync> serde::Serialize for dyn ImplicitOperation<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut ser = serializer.serialize_map(Some(1))?;
        let type_info = self.operation_name();
        ser.serialize_entry(type_info, &Wrap(self))?;
        ser.end()
    }
}

// Deserialize function

struct Wrap<'a, T: ?Sized>(pub &'a T);
impl<'a, T> serde::Serialize for Wrap<'a, T>
where
    T: ?Sized + erased_serde::Serialize + 'a,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.0, serializer)
    }
}

impl<'de, T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static + Pi>
    serde::Deserialize<'de> for Box<dyn ImplicitFunction<T>>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = FunctionVisitor::new();
        deserializer.deserialize_map(visitor)
    }
}

struct FunctionVisitor<T: Float + Send + Sync> {
    _phantom: PhantomData<T>,
}

impl<T: Float + Send + Sync> FunctionVisitor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'de, T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static + Pi>
    serde::de::Visitor<'de> for FunctionVisitor<T>
{
    type Value = Box<dyn ImplicitFunction<T>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Trait object 'dyn Trait'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let type_info = map.next_key::<String>()?.ok_or(serde::de::Error::custom(
            "Expected externally tagged 'dyn Trait'",
        ))?;
        let deserialize_fn = function_runtime_reflection(&type_info).ok_or(
            serde::de::Error::custom(format!("Unknown type for 'dyn Trait': {type_info}")),
        )?;
        let boxed_trait_object: Box<dyn ImplicitFunction<T>> =
            map.next_value_seed(FunctionTypeVisitor { deserialize_fn })?;
        Ok(boxed_trait_object)
    }
}

struct FunctionTypeVisitor<'de, T: Float> {
    deserialize_fn: DeserializeFunctionFn<'de, T>,
}

impl<'de, T: Float> serde::de::DeserializeSeed<'de> for FunctionTypeVisitor<'de, T> {
    type Value = Box<dyn ImplicitFunction<T>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
        let deserialize_fn = self.deserialize_fn;
        deserialize_fn(&mut erased).map_err(|e| serde::de::Error::custom(e))
    }
}

type DeserializeFunctionFn<'de, T> = fn(
    &mut dyn erased_serde::Deserializer<'de>,
) -> erased_serde::Result<Box<dyn ImplicitFunction<T>>>;
fn function_runtime_reflection<
    'de,
    T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static + Pi,
>(
    type_info: &str,
) -> Option<DeserializeFunctionFn<'de, T>> {
    match FunctionComponent::from_str(type_info) {
        Ok(component) => match component {
            FunctionComponent::Gyroid => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Gyroid<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::SchwarzP => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: SchwarzP<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::Neovius => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Neovius<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::XDomain => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: XDomain<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::YDomain => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: YDommain<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::ZDomain => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: ZDomain<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::XYZValue => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: XYZValue = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::Sphere => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Sphere<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::Torus => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Torus<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::Plane => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Plane<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::BoundingBox => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: BoundingBox<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::Capsule => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: Capsule<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::MeshFile => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: MeshFile<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
            FunctionComponent::CustomMesh => {
                let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
                    let s: CustomMesh<T> = erased_serde::deserialize(deserializer)?;
                    let boxed_trait_object: Box<dyn ImplicitFunction<T>> = Box::new(s);
                    Ok(boxed_trait_object)
                };
                Some(deserialize_fn)
            }
        },
        Err(_) => None,
    }
}

// Deserialize operations

impl<'de, T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static> serde::Deserialize<'de>
    for Box<dyn ImplicitOperation<T>>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = OperationVisitor::new();
        deserializer.deserialize_map(visitor)
    }
}

struct OperationVisitor<T: Float + Send + Sync> {
    _phantom: PhantomData<T>,
}

impl<T: Float + Send + Sync> OperationVisitor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'de, T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static> serde::de::Visitor<'de>
    for OperationVisitor<T>
{
    type Value = Box<dyn ImplicitOperation<T>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Trait object 'dyn Trait'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let type_info = map.next_key::<String>()?.ok_or(serde::de::Error::custom(
            "Expected externally tagged 'dyn Trait'",
        ))?;
        let deserialize_fn = operation_runtime_reflection(&type_info).ok_or(
            serde::de::Error::custom(format!("Unknown type for 'dyn Trait': {type_info}")),
        )?;
        let boxed_trait_object: Box<dyn ImplicitOperation<T>> =
            map.next_value_seed(OperationTypeVisitor { deserialize_fn })?;
        Ok(boxed_trait_object)
    }
}

struct OperationTypeVisitor<'de, T: Float> {
    deserialize_fn: DeserializeOperationFn<'de, T>,
}

impl<'de, T: Float> serde::de::DeserializeSeed<'de> for OperationTypeVisitor<'de, T> {
    type Value = Box<dyn ImplicitOperation<T>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
        let deserialize_fn = self.deserialize_fn;
        deserialize_fn(&mut erased).map_err(|e| serde::de::Error::custom(e))
    }
}

type DeserializeOperationFn<'de, T> = fn(
    &mut dyn erased_serde::Deserializer<'de>,
) -> erased_serde::Result<Box<dyn ImplicitOperation<T>>>;

fn operation_runtime_reflection<
    'de,
    T: Float + Send + Sync + Serialize + Deserialize<'de> + 'static,
>(
    type_info: &str,
) -> Option<DeserializeOperationFn<'de, T>> {
    if type_info == "Add" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Add = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "Subtract" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Subtract = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "Multiply" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Multiply = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "Divide" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Divide = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "LinearInterpolation" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: LinearInterpolation<T> = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "BooleanUnion" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: BooleanUnion = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "BooleanDifference" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: BooleanDifference = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "BooleanIntersection" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: BooleanIntersection = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "Offset" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Offset<T> = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else if type_info == "Thickness" {
        let deserialize_fn = |deserializer: &mut dyn erased_serde::Deserializer<'de>| {
            let s: Thickness<T> = erased_serde::deserialize(deserializer)?;
            let boxed_trait_object: Box<dyn ImplicitOperation<T>> = Box::new(s);
            Ok(boxed_trait_object)
        };
        Some(deserialize_fn)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::types::computation::{
        components::{function_components::FUNCTION_COMPONENTS, operation_components::OPERATION_COMPONENTS}, ImplicitModel,
    };

    use super::*;

    #[test]
    fn test_serialize_deserialize_functions() {
        let mut model: ImplicitModel<f32> = ImplicitModel::new();
        let mut tags: Vec<String> = Vec::new();

        for func in FUNCTION_COMPONENTS {
            let component = func.create_default();
            let tag = model
                .add_component(component.type_name(), component)
                .unwrap();
            tags.push(tag);
        }

        let model_json = serde_json::to_string_pretty(&model).unwrap();
        let deserialized_model: ImplicitModel<f32> = serde_json::from_str(&model_json).unwrap();

        for tag in &tags {
            assert!(
                deserialized_model.get_component(tag).is_some(),
                "Component with tag '{}' is missing in the deserialized model",
                tag
            );
        }
    }

    #[test]
    fn test_serialize_deserialize_operations() {
        let mut model: ImplicitModel<f32> = ImplicitModel::new();
        let mut tags: Vec<String> = Vec::new();

        for func in OPERATION_COMPONENTS {
            let component = func.create_default();
            let tag = model
                .add_component(component.type_name(), component)
                .unwrap();
            tags.push(tag);
        }

        let model_json = serde_json::to_string_pretty(&model).unwrap();
        let deserialized_model: ImplicitModel<f32> = serde_json::from_str(&model_json).unwrap();

        for tag in &tags {
            assert!(
                deserialized_model.get_component(tag).is_some(),
                "Component with tag '{}' is missing in the deserialized model",
                tag
            );
        }
    }
}
