//! # Computation Module
//!
//! The `computation` module provides the core tools and abstractions for building, manipulating, and evaluating implicit models. These models represent continuous scalar fields in 3D space and can be used to define and polygonize complex geometries.
//!
//! ## Overview
//!
//! At the heart of this module is the [`ImplicitModel`](crate::types::computation::model::ImplicitModel), a computation graph composed of reusable components. This graph defines spatial relationships, transformations, and operations on scalar fields, enabling a flexible approach to implicit modeling.
//!
//! ### Features
//!
//! - **Triply-Periodic Surfaces (TPS)**: Predefined implicit functions for advanced modeling.
//! - **Custom Components**: Easily extend functionality by implementing new functions or operations.
//! - **Extensibility**: Seamlessly integrate geometric primitives and user-defined components within the computation graph.
//! 
//! ## Components of an Implicit Model
//!
//! The computation graph in an `ImplicitModel` consists of interconnected components:
//!
//! ### 1. **Functions**
//!
//! Functions represent mappings from `{x, y, z}` coordinates to scalar values, commonly used to define distance functions or field equations.
//!
//! - **Implementation**: Functions are implemented as structs that satisfy the [`ImplicitFunction`](crate::types::computation::traits::ImplicitFunction) trait.
//! - **Examples**: Many primitives in the [`geometry`](crate::types::geometry) module, such as [`Sphere`](crate::types::geometry::Sphere) and [`Torus`](crate::types::geometry::Torus), can be used as implicit functions.
//! - **Enumeration**: The available functions are listed in the [`FunctionComponent`](crate::types::computation::functions::FunctionComponent) enum.
//!
//! ### 2. **Operations**
//!
//! Operations process the outputs of other components to create new behaviors or combine scalar fields. These components allow for:
//! - **Boolean Operations**: Combine geometries using union, intersection, and subtraction.
//! - **Arithmetic Operations**: Apply transformations like multiplication, addition, or negation to scalar values.
//! - **Implementation**: Operations must implement the [`ImplicitOperation`](crate::types::computation::traits::ImplicitOperation) trait.
//!
//! ### 3. **Constants**
//!
//! Constants represent fixed scalar values within the computation graph. These components provide reusable inputs for other functions and operations without modification.
//!
//! ## Extending the Module
//!
//! To define new behavior, implement the following traits:
//! - [`ImplicitFunction`](crate::types::computation::traits::ImplicitFunction) for custom spatial functions.
//! - [`ImplicitOperation`](crate::types::computation::traits::ImplicitOperation) for custom data transformations.

/// Error types related to model computation.
mod model_error;

/// Utilities for scalar field manipulation and evaluation.
mod scalar_field;

/// Predefined functions, including triply-periodic surfaces (TPS).
pub mod functions;

/// Tools for constructing and managing computation graphs (e.g., `ImplicitModel`).
pub mod model;

/// Arithmetic and boolean operations for scalar field manipulation.
pub mod operations;

/// Traits for defining custom implicit functions and operations.
pub mod traits;

pub use model_error::*;
pub use scalar_field::*;
