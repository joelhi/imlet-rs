use std::fmt::Debug;

use num_traits::Float;

pub trait SignedDistance<T: Float + Debug + Send + Sync>: Sync + Send {
    fn signed_distance(&self, x: T, y: T, z: T) -> T;
}