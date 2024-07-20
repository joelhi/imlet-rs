use std::fmt::Debug;

use num_traits::Float;

use crate::types::geometry::{BoundingBox, Vec3};

pub trait SpatialQuery<T: Float + Debug + Send + Sync>: Copy {
    fn bounds(&self) -> BoundingBox<T>;

    fn closest_point(&self, query_point: &Vec3<T>) -> Vec3<T>;

    fn default() -> Self;
}

pub trait SignedQuery<T: Float + Debug + Send + Sync>: SpatialQuery<T> {
    fn normal_at(&self, query_point: &Vec3<T>) -> Vec3<T>;
}
