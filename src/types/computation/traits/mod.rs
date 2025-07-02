mod implicit_functions;

pub use implicit_functions::*;

/// Default trait for a float value in an implicit model.
#[cfg(feature = "serde")]
pub trait ModelFloat:
    num_traits::Float + Concurrency + serde::Serialize + crate::utils::math_helper::Pi
{
}

/// Default trait for a float value in an implicit model.
#[cfg(not(feature = "serde"))]
pub trait ModelFloat: num_traits::Float + Concurrency + crate::utils::math_helper::Pi {}

// Blanket implementation for all types that satisfy the bounds
#[cfg(feature = "serde")]
#[doc(hidden)]
impl<T> ModelFloat for T where
    T: num_traits::Float + Concurrency + serde::Serialize + crate::utils::math_helper::Pi
{
}

// Blanket implementation for all types that satisfy the bounds
#[cfg(not(feature = "serde"))]
#[doc(hidden)]
impl<T> ModelFloat for T where T: num_traits::Float + Concurrency + crate::utils::math_helper::Pi {}

#[doc(hidden)]
pub trait Concurrency: Send + Sync {}
#[doc(hidden)]
impl<T: Send + Sync> Concurrency for T {}
