use std::fmt::Debug;

use num_traits::Float;

use crate::types::{
    computation::{
        components::{Data, Parameter},
        traits::ImplicitFunction,
    },
    geometry::{traits::SignedDistance, GeometryCollection, Mesh, Triangle},
};

/// Distance function for an arbitrary geometry type.
#[derive(Debug)]
pub struct CustomGeometry<Q, T> {
    /// Geometry to use for signed distance computation
    pub geometry: Q,
    /// Additional offset applied to the distance field.
    pub offset: T,
}

impl<Q, T: Float> CustomGeometry<Q, T> {
    /// Create a new custom sdf from a geometry that implements the SignedDistance trait.
    ///
    /// # Arguments
    ///
    /// * `geometry` - Geomtry to use as base for signed distance computation.
    pub fn new(geometry: Q) -> Self {
        Self {
            geometry,
            offset: T::zero(),
        }
    }
    /// Create a new custom sdf from a geometry that implements the SignedDistance trait, with an additional offset.
    ///
    /// This offset can be useful if the geometry type has no inside, like a Line or a Vec3. Then the offset will define the thickness.
    ///
    /// # Arguments
    ///
    /// * `geometry` - Geomtry to use as base for signed distance computation.
    /// * `offset` - Additional offset applied to the distance field.
    pub fn with_offset(geometry: Q, offset: T) -> Self {
        Self { geometry, offset }
    }
}

impl<T: Float> CustomGeometry<GeometryCollection<Triangle<T>, T>, T> {
    /// Create a custom distance function based on a collection of the triangles in a mesh.
    pub fn from_mesh(mesh: &Mesh<T>) -> Self {
        let collection = GeometryCollection::build(mesh.as_triangles());

        Self::new(collection)
    }
}

impl<Q: SignedDistance<T> + Send + Sync, T: Float + Send + Sync> ImplicitFunction<T>
    for CustomGeometry<Q, T>
{
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.geometry.signed_distance(x, y, z) - self.offset
    }

    fn parameters(&self) -> &[Parameter] {
        &[]
    }

    fn set_parameter(&mut self, _: &str, _: Data<T>) {
        // Void
    }

    fn read_parameter(&self, _: &str) -> Option<Data<T>> {
        None
    }

    fn function_name(&self) -> &'static str {
        "CustomGeometry"
    }
}
