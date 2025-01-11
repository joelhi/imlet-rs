use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::geometry::BoundingBox;

/// Config for model definition and computation.
///
/// The struct is specifying meta parameters such as geometric bounds and post processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig<T> {
    /// Geometric limits of the model.
    pub bounds: BoundingBox<T>,
    /// Cap object at bounds
    pub cap: bool,
    /// Number of smoothing iterations on the field.
    pub smoothing_iter: u32,
    /// Laplacian smoothing factor
    pub smoothing_factor: T,
}

impl<T: Float> ModelConfig<T> {
    /// Create a new config with a certain domain.
    pub fn new(bounds: BoundingBox<T>) -> Self {
        Self {
            bounds,
            cap: true,
            smoothing_iter: 0,
            smoothing_factor: T::from(0.75).expect("Should be able to convert factor 0.75 to T"),
        }
    }

    /// Create a new config with a certain domain and field smoothing
    pub fn with_smoothing(
        bounds: BoundingBox<T>,
        smoothing_iter: u32,
        smoothing_factor: T,
    ) -> Self {
        Self {
            bounds,
            cap: true,
            smoothing_iter,
            smoothing_factor,
        }
    }
}
