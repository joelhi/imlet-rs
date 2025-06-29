mod dense_field;
/// Module with types and traits for structure agnostic iteration of field data.
pub mod field_iterator;
/// Module with types used for sampling discrete fields.
pub mod sampler;
mod sparse_field;

pub use dense_field::*;
pub use sparse_field::*;