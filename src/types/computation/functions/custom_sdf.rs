use std::fmt::Debug;

use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::types::{
    computation::{
        components::{Data, Parameter},
        traits::ImplicitFunction,
    },
    geometry::{traits::SignedDistance, Mesh, Octree, Triangle, Vec3},
};

/// Distance function for an arbitrary geometry type.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomMesh<T> {
    /// Geometry to use for signed distance computation
    pub octree: Option<Octree<Triangle<T>, T>>,
    /// Additional offset applied to the distance field.
    pub offset: T,
}

impl<T: Float> CustomMesh<T> {
    /// Create a new empty custom mesh container.
    pub fn new() -> Self {
        Self {
            octree: None,
            offset: T::zero(),
        }
    }

    /// Create a custom distance function based on a collection of the triangles in a mesh.
    pub fn build(mesh: &Mesh<T>) -> Self {
        Self {
            octree: Some(mesh.compute_octree(10, 12)),
            offset: T::zero(),
        }
    }

    /// Create a new custom distance function with a specific offset
    pub fn with_offset(mesh: &Mesh<T>, offset: T) -> Self {
        Self {
            octree: Some(mesh.compute_octree(10, 12)),
            offset: offset,
        }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for CustomMesh<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        if let Some(geometry) = &self.octree {
            return geometry.signed_distance(&Vec3::new(x, y, z)) - self.offset;
        }

        T::zero()
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {
        // Void. Need to figure out how to offer the option to set at runtime.
    }

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn function_name(&self) -> &'static str {
        "CustomMesh"
    }
}
