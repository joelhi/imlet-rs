use std::fmt::Debug;

use num_traits::Float;

use crate::types::computation::ImplicitFunction;
use crate::utils::math_helper::normalize;

#[derive(Debug, Clone, Copy)]
pub struct ZDomain<T: Float + Debug> {
    min: T,
    max: T
 }

impl<T: Float + Debug> ZDomain<T> {
    pub fn remapped(min: T, max: T) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn natural()->Self{
        Self{
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for ZDomain<T> {
    fn eval(&self, _: T, _: T, z: T) -> T {
        normalize(z, self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct YDomain<T: Float + Debug> {
    min: T,
    max: T
 }

impl<T: Float + Debug> YDomain<T> {
    pub fn remapped(min: T, max: T) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn natural()->Self{
        Self{
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for YDomain<T> {
    fn eval(&self, _: T, y: T, _: T) -> T {
        normalize(y, self.min, self.max)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XDomain<T: Float + Debug> {
    min: T,
    max: T
 }

impl<T: Float + Debug> XDomain<T> {
    pub fn remapped(min: T, max: T) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn natural()->Self{
        Self{
            min: T::zero(),
            max: T::one(),
        }
    }
}

impl<T: Float + Debug + Send + Sync> ImplicitFunction<T> for XDomain<T> {
    fn eval(&self, x: T, _: T, _: T) -> T {
        normalize(x, self.min, self.max)
    }
}