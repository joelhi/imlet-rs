use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::computation::traits::ImplicitFunction;
use crate::utils::math_helper::normalize;

/// Distance function that evaluates to the z-coordinate
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ZCoord<T: Float + Debug> {
    min: T,
    max: T,
}

impl<T: Float + Debug> ZCoord<T> {
    /// Create a distance function for a remapped z domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular z coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for ZCoord<T> {
    fn eval(&self, _: T, _: T, z: T) -> T {
        normalize(z, self.min, self.max)
    }
}

/// Distance function that evaluates to the y-coordinate
#[derive(Debug, Clone, Copy)]
pub struct YCoord<T: Float + Debug> {
    min: T,
    max: T,
}

impl<T: Float + Debug> YCoord<T> {
    /// Create a distance function for a remapped y domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular y coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for YCoord<T> {
    fn eval(&self, _: T, y: T, _: T) -> T {
        normalize(y, self.min, self.max)
    }
}

/// Distance function that evaluates to the x-coordinate
#[derive(Debug, Clone, Copy)]
pub struct XCoord<T: Float + Debug> {
    min: T,
    max: T,
}

impl<T: Float + Debug> XCoord<T> {
    /// Create a distance function for a remapped x domain.
    ///
    /// Can be used for interpolation.
    ///
    /// # Arguments
    ///
    /// * `min` - Coordinate value that maps to 0.
    /// * `max` - Coordinate value that maps to 1.
    pub fn remapped(min: T, max: T) -> Self {
        Self { min, max }
    }

    /// Function returning the regular x coordinate at any point.
    pub fn natural() -> Self {
        Self {
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for XCoord<T> {
    fn eval(&self, x: T, _: T, _: T) -> T {
        normalize(x, self.min, self.max)
    }
}
