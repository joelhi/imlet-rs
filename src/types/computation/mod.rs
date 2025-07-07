//! # Computation Module
//!
//! The `computation` module provides the core tools and abstractions for building, manipulating, and evaluating implicit models.
//! These models represent continuous scalar fields in 3D space and can be used to define and polygonize complex geometries.
//!
//! ## Architecture Overview
//!
//! The computation system is built around a directed acyclic graph (DAG) structure where:
//! - Nodes represent computational components
//! - Edges represent data flow between components
//! - Evaluation follows topological ordering
//!
//! ## Core Components
//!
//! The computation graph in an [`ImplicitModel`] consists of three types of nodes:
//!
//! 1. **[`Functions`]**: Transform spatial coordinates to scalar values
//!    - Implement [`ImplicitFunction`] trait
//!    - No dependencies on other components
//!    - Examples: distance fields, periodic functions
//!    - Evaluated as `f(x, y, z) -> scalar`
//!
//! 2. **[`Operations`]**: Process scalar inputs to produce new values
//!    - Implement [`ImplicitOperation`] trait
//!    - Can depend on multiple inputs
//!    - Examples: boolean operations, arithmetic
//!    - Evaluated as `f(inputs[]) -> scalar`
//!
//! 3. **[`Constants`]**: Provide fixed scalar values
//!    - Examples: thresholds, scale factors
//!    - No dependencies
//!
//! ## Evaluation Process
//!
//! 1. **Graph Compilation**:
//!    - Validates component connections
//!    - Detects cyclic dependencies
//!    - The output is computed for a selected component (by default the last added)
//!    - Only evaluates components needed for the selected component
//!
//! 2. **Field Sampling**:
//!    - Discretizes continuous field into grid points
//!    - Supports both dense and sparse strategies
//!    - Parallelizes evaluation across regions
//!
//! 3. **Isosurface Extraction**:
//!    - Uses marching cubes to polygonize the scalar field
//!    - Works uniformly for both sparse and dense fields
//!    - Generates triangle mesh output
//!    - Supports normal computation for rendering
//!
//! ## Performance Considerations
//!
//! The performance of implicit modeling is primarily determined by:
//!
//! 1. **Sampling Strategy**:
//!    - Dense sampling evaluates every point in the domain
//!    - Sparse sampling focuses on regions near the surface
//!    - Choose based on your domain size and required detail
//!
//! 2. **Component Selection**:
//!    - Each component type has different evaluation costs
//!    - Distance functions from custom objects are more expensive than simple primitives
//!    - Boolean operations are generally fast
//!
//! 3. **Resource Usage**:
//!    - Memory usage scales with domain size and sampling resolution
//!    - Large domains may require sparse sampling to be practical
//!
//! ## Error Handling
//!
//! The system uses [`ModelError`] to handle various failure cases:
//!
//! 1. **Graph Structure Errors**:
//!    - `CyclicDependency` - A component depends on itself (directly or indirectly)
//!    - `MissingTag` - Referenced component tag doesn't exist
//!    - `DuplicateTag` - Attempted to add component with already used tag
//!
//! 2. **Input Errors**:
//!    - `InputIndexOutOfRange` - Input index larger than component's input count
//!    - `IncorrectInputCount` - Wrong number of inputs provided to operation
//!    - `MissingInput` - Required input not connected
//!
//! 3. **Configuration Errors**:
//!    - `MissingOutput` - No output component specified for evaluation
//!    - `MissingConfig` - Required configuration not provided
//!    - `MissingRequiredParam` - Required parameter missing from builder
//!    - `IncorrectDataSize` - Data buffer size doesn't match field dimensions
//!    - `NoDefaultOutput` - No default output component set
//!
//! Custom error messages can also be created using the `Custom` variant.
//!
//! ## Extending the System
//!
//! To add new functionality:
//!
//! 1. **Custom Functions**:
//!    - Implement [`ImplicitFunction`] trait
//!    - Define spatial mapping
//!    - Handle boundary cases
//!
//! 2. **Custom Operations**:
//!    - Implement [`ImplicitOperation`] trait
//!    - Define input requirements
//!    - Ensure thread safety
//!
//! > ⚠️ **Note**: Due to Rust's lack of runtime reflection, deserialization of custom implementations
//! > of [`ImplicitFunction`] and [`ImplicitOperation`] is not currently supported.
//!
//! [`ImplicitModel`]: model::ImplicitModel
//! [`Functions`]: functions::FunctionComponent
//! [`Operations`]: operations::OperationComponent
//! [`ImplicitFunction`]: traits::ImplicitFunction
//! [`ImplicitOperation`]: traits::ImplicitOperation
//! [`ModelError`]: model_error::ModelError

/// Error types related to model computation.
mod model_error;

/// Types related to discrete scalar fields and sampling models.
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
