mod implicit_functions;

pub use implicit_functions::*;

/// Default trait for a float value in an implicit model.
pub trait ModelFloat:
    num_traits::Float + Send + Sync + serde::Serialize + crate::utils::math_helper::Pi
{
}

// Blanket implementation for all types that satisfy the bounds
impl<T> ModelFloat for T where
    T: num_traits::Float + Send + Sync + serde::Serialize + crate::utils::math_helper::Pi
{
}
