use crate::types::computation::model::{Data, Parameter};
use std::any::type_name;

/// Trait to define a distance function in 3d space.
///
/// This trait provides the framework for evaluating distance functions as part of an implicit model.
/// A struct that implements this trait can be passed to the [`ImplicitModel`](crate::types::computation::model::ImplicitModel) via the [`add_function`](crate::types::computation::model::ImplicitModel::add_function) method.
///
/// # Example
///
/// Example implementations can be found in the [`geometry`](crate::types::geometry) module. For example, for a sphere, it would look something like this:
///
/// ```rust
/// # use imlet::types::computation::traits::{ModelFloat, ImplicitFunction, ImplicitComponent};
/// # use imlet::types::geometry::Vec3;
/// # use num_traits::Float;
/// # use std::marker::{Send, Sync};
/// #
/// # #[derive(Debug, Clone, Copy)]
/// # pub struct Sphere<T>{ centre: Vec3<T>, radius: T};
///
/// // Default implementation of base trait.
/// impl<T: ModelFloat> ImplicitComponent<T> for Sphere<T>{};
///
/// impl<T: ModelFloat> ImplicitFunction<T> for Sphere<T> {
///     fn eval(&self, x: T, y: T, z: T) -> T {
///         self.centre.distance_to_coord(x, y, z) - self.radius
///     }
/// }
/// ```
pub trait ImplicitFunction<T>: ImplicitComponent<T> {
    /// Evaluate a function in 3 dimensional space. *f(x,y,z)->value*
    ///
    /// This function will be evaluated at each sample point in an implicit model.
    /// # Arguments
    ///
    /// * `x` - X coordinate to evaluate.
    /// * `y` - Y coordinate to evaluate.
    /// * `z` - Z coordinate to evaluate.
    fn eval(&self, x: T, y: T, z: T) -> T;
}

/// Trait to define an operation to be performed as part of an implicit model computation.
///
/// This is used to define custom operations on data in an implicit model, independent of global coordinates.
/// A struct that implements this trait can be passed to the [`ImplicitModel`](crate::types::computation::model::ImplicitModel) via the [`add_operation`](crate::types::computation::model::ImplicitModel::add_operation) method.
///
/// # Example
///
/// Examples can be found in the [`computation::operations`](crate::types::computation::operations) module, for example a simple addition would look like this:
///
/// ```rust
/// # use imlet::types::computation::traits::{ModelFloat, ImplicitOperation, ImplicitComponent};
/// # #[derive(Debug, Clone, Copy)]
/// # pub struct Add;
///
/// static INPUT_NAMES: [&str; 2] = ["First Number", "Second Number"];
/// // Default implementation of base trait    .
/// impl<T> ImplicitComponent<T> for Add{};
///
/// impl<T: ModelFloat> ImplicitOperation<T> for Add {
///     fn eval(&self, inputs: &[T]) -> T {
///         inputs[0] + inputs[1]
///     }
///
///     fn inputs(&self) -> &[&str] {
///         &INPUT_NAMES
///     }
/// }
///
/// ```
pub trait ImplicitOperation<T>: ImplicitComponent<T> {
    /// Perform the operation based on the input values.
    /// # Arguments
    ///
    /// * `inputs` - Inputs for operation, passed from components in implicit model.
    fn eval(&self, inputs: &[T]) -> T;

    /// Communicates to the model the names of and number of inputs to this operation.
    fn inputs(&self) -> &[&str];
}
/// Trait for general functionality of an implicit component.
///
/// The trait offers the ability to expose parameters, which can be manipulated at runtime.
/// By default nothing is exposed and nothing has to be implemented, but it is recommended to implement the `name` function.
pub trait ImplicitComponent<T>: Root {
    /// Declare variable parameters for the component.
    ///
    /// If no parameters are applicable, this can just return an empty array.
    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    /// Process the input from one of the declared public parameters.
    ///
    /// The provided value should be assigned where intended, using the mutable reference to self.
    ///
    /// If there are no parameters exposed, this shoudn't do anything.
    fn set_parameter(&mut self, _parameter_name: &str, _data: Data<T>) {}

    /// Read the value of a parameter.
    fn read_parameter(&self, _parameter_name: &str) -> Option<Data<T>> {
        None
    }

    /// Name of the component.
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

// 2) Serde helper: only adds erased_serde::Serialize when the `serde` feature is on
#[cfg(feature = "serde")]
#[doc(hidden)]
pub trait SerdeComponent: erased_serde::Serialize {}
#[cfg(feature = "serde")]
impl<T: erased_serde::Serialize> SerdeComponent for T {}

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
pub trait SerdeComponent {}
#[cfg(not(feature = "serde"))]
#[doc(hidden)]
impl<T> SerdeComponent for T {}

// 3) Root alias: always present, but expands to whatever the two helpers demand
#[doc(hidden)]
pub trait Root: super::Concurrency + SerdeComponent {}
#[doc(hidden)]
impl<T: super::Concurrency + SerdeComponent> Root for T {}
