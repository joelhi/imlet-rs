mod model_error;
mod scalar_field;

/// Various functions, including Triply-Periodic Surfaces (TPS).
pub mod functions;
/// Tools related to building an implicit computation model.
pub mod model;
/// Operations on data such as basic arithmetic and boolean operations.
pub mod operations;
/// Traits used to define implicit operations and functions.
pub mod traits;

pub use model_error::*;
pub use scalar_field::*;
