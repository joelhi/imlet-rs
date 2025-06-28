//! # Computation Module
//!
//! The `computation` module provides the core tools and abstractions for building, manipulating, and evaluating implicit models. These models represent continuous scalar fields in 3D space and can be used to define and polygonize complex geometries.
//!
//! At the heart of this module is the [`ImplicitModel`](crate::types::computation::model::ImplicitModel), a computation graph composed of reusable components. This graph defines distance functions, and operations representing the scalar fields.
//!
//! ## Components of an Implicit Model
//!
//! The computation graph in an `ImplicitModel` consists of connected [`ModelComponent`](crate::types::computation::model::ModelComponent).
//! There are three difference component types that make up a model, these are:
//!
//! ### 1. **[`Functions`](crate::types::computation::model::ModelComponent::Function)**
//!
//! Functions represent mappings from `{x, y, z}` coordinates to scalar values, commonly used to define distance functions or field equations.
//!
//! - **Implementation**: A `Function` component cam contain any struct that implement the [`ImplicitFunction`](crate::types::computation::traits::ImplicitFunction) trait.
//! - **Primitives**: Many primitives in the [`geometry`](crate::types::geometry) module, such as [`Sphere`](crate::types::geometry::Sphere) and [`Torus`](crate::types::geometry::Torus), can be used as implicit functions.
//!
//! The available functions are listed in the [`FunctionComponent`](crate::types::computation::functions::FunctionComponent) enum.
//!
//! ### 2. **[`Operations`](crate::types::computation::model::ModelComponent::Operation)**
//!
//! Operations process the outputs of other components to create new behaviors or combine scalar fields. These components allow for:
//!
//! - **Boolean Operations**: Combine geometries using union, intersection, and subtraction.
//! - **Arithmetic Operations**: Apply transformations like multiplication or addition to scalar values.
//! - **Implementation**: A `Operation` component cam contain any struct that implement the [`ImplicitOperation`](crate::types::computation::traits::ImplicitOperation) trait.
//!
//! The available operations are listed in the [`OperationComponent`](crate::types::computation::operations::OperationComponent) enum.
//!
//! ### 3. **[`Constants`](crate::types::computation::model::ModelComponent::Constant)**
//!
//! Constants represent fixed scalar values within the computation graph. These components provide reusable inputs for other functions and operations.
//!
//! ## Extending the Module
//!
//! To define new behavior, implement the following traits:
//! - [`ImplicitFunction`](crate::types::computation::traits::ImplicitFunction) for custom spatial functions.
//! - [`ImplicitOperation`](crate::types::computation::traits::ImplicitOperation) for custom data transformations.
//!
//! > ⚠️ **Disclaimer**:
//! >
//! > Due to rusts lack of runtime reflection it's currently not possible to deserialize custom [`ImplicitFunction`](crate::types::computation::traits::ImplicitFunction) and [`ImplicitFunction`](crate::types::computation::traits::ImplicitOperation) structs at this point in time.

/// Error types related to model computation.
mod model_error;

pub mod data;

/// Predefined functions, including triply-periodic surfaces (TPS).
pub mod functions;

/// Tools for constructing and managing computation graphs (e.g., `ImplicitModel`).
pub mod model;

/// Arithmetic and boolean operations for scalar field manipulation.
pub mod operations;

/// Traits for defining custom implicit functions and operations.
pub mod traits;

pub use model_error::*;
