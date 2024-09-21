mod component;
mod computation_graph;
mod implicit_model;
mod scalar_field;

/// Various distance functions, including Triply-Periodic Surfaces (TPS), primitives and custom meshes.
pub mod distance_functions;
/// Operations on implicit data such as basic arithmetic and boolean operations.
pub mod operations;
/// Traits used to define implicit operations and functions.
pub mod traits;

pub(crate) use computation_graph::*;
pub use implicit_model::*;
pub use scalar_field::*;
