use std::fmt::Debug;

use log::error;
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::computation::{
    model::{Data, DataType, Parameter},
    traits::{ImplicitComponent, ImplicitFunction, ModelFloat},
};

use super::{
    traits::{SignedDistance, SpatialQuery},
    Line, Triangle, Vec3,
};

/// An axis-aligned bounding box.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox<T> {
    // Minimum coordinate of the box
    pub min: Vec3<T>,
    // Maximum coordinate of the box
    pub max: Vec3<T>,
}

impl<T> BoundingBox<T> {
    /// Create a new BoundingBox from a min and max coordinate.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum coordinate of the Box.
    /// * `max` - The maximum coordinate of the Box.
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self { min, max }
    }
}

impl<T: Float> BoundingBox<T> {
    /// Create a new BoundingBox with zero size at the origin.
    pub fn zero() -> Self {
        Self {
            min: Vec3::origin(),
            max: Vec3::origin(),
        }
    }

    pub fn union(&self, other: &BoundingBox<T>) -> BoundingBox<T> {
        Self {
            min: self.min.min(&other.min),
            max: self.max.max(&other.max),
        }
    }

    /// Return the size of the box in x, y and z
    ///
    /// # Returns
    ///
    /// * `(x_size, y_size, z_size)` - A tuple with the size in x, y and z.
    pub fn dimensions(&self) -> (T, T, T) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    /// Checks if the box contains a point
    ///
    /// # Arguments
    ///
    /// * `point` - Point for contains check.
    pub fn contains(&self, point: &Vec3<T>) -> bool {
        point.x >= self.min.x
            && point.y >= self.min.y
            && point.z >= self.min.z
            && point.x <= self.max.x
            && point.y <= self.max.y
            && point.z <= self.max.z
    }

    /// Checks if the box contains a point defined by a x, y and z coordinate.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the point.
    /// * `y` - Y coordinate of the point.
    /// * `z` - Z coordinate of the point.
    pub fn contains_coord(&self, x: T, y: T, z: T) -> bool {
        x >= self.min.x
            && y >= self.min.y
            && z >= self.min.z
            && x <= self.max.x
            && y <= self.max.y
            && z <= self.max.z
    }

    /// Returns the 8 corners of the box as 3d points.
    ///
    /// ```text
    ///      4 -------- 7       Z
    ///     /|         /|       |
    ///    / |        / |       + -- Y
    ///   5 -------- 6  |      /
    ///   |  |       |  |     X
    ///   |  0 ------|-- 3    
    ///   | /        | /    
    ///   |/         |/   
    ///   1 -------- 2      
    /// ```
    ///
    pub fn corners(&self) -> [Vec3<T>; 8] {
        let delta = self.max - self.min;
        [
            self.min,
            self.min + Vec3::new(delta.x, T::zero(), T::zero()),
            self.min + Vec3::new(delta.x, delta.y, T::zero()),
            self.min + Vec3::new(T::zero(), delta.y, T::zero()),
            self.min + Vec3::new(T::zero(), T::zero(), delta.z),
            self.min + Vec3::new(delta.x, T::zero(), delta.z),
            self.max,
            self.min + Vec3::new(T::zero(), delta.y, delta.z),
        ]
    }

    /// Returns the wirframe of the box as a list of 12 lines.
    pub fn as_wireframe(&self) -> [Line<T>; 12] {
        let corners = self.corners();
        [
            Line::new(corners[0], corners[1]),
            Line::new(corners[1], corners[2]),
            Line::new(corners[2], corners[3]),
            Line::new(corners[3], corners[0]),
            Line::new(corners[4], corners[5]),
            Line::new(corners[5], corners[6]),
            Line::new(corners[6], corners[7]),
            Line::new(corners[7], corners[4]),
            Line::new(corners[0], corners[4]),
            Line::new(corners[1], corners[5]),
            Line::new(corners[2], corners[6]),
            Line::new(corners[3], corners[7]),
        ]
    }

    /// Returns a triangulated box as a list of triangles.
    pub fn as_triangles(&self) -> [Triangle<T>; 12] {
        let corners = self.corners();
        [
            Triangle::new(corners[0], corners[1], corners[2]),
            Triangle::new(corners[0], corners[2], corners[3]),
            Triangle::new(corners[0], corners[1], corners[5]),
            Triangle::new(corners[0], corners[5], corners[4]),
            Triangle::new(corners[1], corners[6], corners[2]),
            Triangle::new(corners[1], corners[5], corners[6]),
            Triangle::new(corners[2], corners[6], corners[7]),
            Triangle::new(corners[2], corners[7], corners[3]),
            Triangle::new(corners[3], corners[0], corners[4]),
            Triangle::new(corners[3], corners[4], corners[7]),
            Triangle::new(corners[7], corners[4], corners[6]),
            Triangle::new(corners[4], corners[5], corners[6]),
        ]
    }

    /// Returns the centre of the box.
    pub fn centroid(&self) -> Vec3<T> {
        (self.max + self.min) * T::from(0.5).expect("Failed to convert number to T")
    }

    /// Checks if the box intersects another box.
    ///
    /// # Arguments
    ///
    /// * `other` - Other box to check for.
    pub fn intersects(&self, other: &BoundingBox<T>) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Get the closest point on the box.
    ///
    /// # Arguments
    ///
    /// * `point` - Point to find closest point from.
    pub fn closest_point(&self, point: &Vec3<T>) -> Vec3<T> {
        let x = point.x.max(self.min.x).min(self.max.x);
        let y = point.y.max(self.min.y).min(self.max.y);
        let z = point.z.max(self.min.z).min(self.max.z);
        Vec3 { x, y, z }
    }

    /// Get the signed distance to the box.
    ///
    /// # Arguments
    ///
    /// * `point` - Point to find the signed distance to.
    pub fn signed_distance(&self, point: &Vec3<T>) -> T {
        let diff1 = self.max - *point;
        let diff2 = self.min - *point;

        let dist = diff1.x.abs().min(
            diff1.y.abs().min(
                diff1
                    .z
                    .abs()
                    .min(diff2.x.abs().min(diff2.y.abs().min(diff2.z.abs()))),
            ),
        );

        if self.contains(point) {
            -dist
        } else {
            dist
        }
    }

    /// Offset the box equally in all directions.
    ///
    /// # Arguments
    ///
    /// * `distance` - Offset distance.
    pub fn offset(&self, distance: T) -> BoundingBox<T> {
        let offset_vec = Vec3::new(distance, distance, distance);
        Self {
            min: self.min - offset_vec,
            max: self.max + offset_vec,
        }
    }

    /// Create a union box containing a collection of objects.
    pub fn from_objects<Q: SpatialQuery<T>>(objects: &[Q]) -> Self {
        let bounds: Vec<BoundingBox<T>> = objects.iter().map(|o| o.bounds()).collect();
        let mut total_extents = bounds[0];
        for bound in bounds {
            total_extents = total_extents.union(&bound);
        }

        total_extents
    }
}

impl<T: ModelFloat> SignedDistance<T> for BoundingBox<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(&Vec3::new(x, y, z))
    }
}

static BOUNDING_BOX_PARAMETERS: [Parameter; 2] = [
    Parameter {
        name: "Min",
        data_type: DataType::Vec3,
    },
    Parameter {
        name: "Max",
        data_type: DataType::Vec3,
    },
];

impl<T: ModelFloat> ImplicitFunction<T> for BoundingBox<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(&Vec3::new(x, y, z))
    }
}

impl<T: ModelFloat> ImplicitComponent<T> for BoundingBox<T> {
    fn parameters(&self) -> &[Parameter] {
        &BOUNDING_BOX_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        if !(Parameter::set_vec3_from_param(parameter_name, &data, "Min", &mut self.min)
            || Parameter::set_vec3_from_param(parameter_name, &data, "Max", &mut self.max))
        {
            error!("Unknown parameter name: {}", parameter_name);
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "Min" => Some(Data::Vec3(self.min)),
            "Max" => Some(Data::Vec3(self.max)),
            _ => None,
        }
    }

    fn name(&self) -> &'static str {
        "BoundingBox"
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        types::geometry::{traits::Bounded, Mesh},
        utils::io::parse_obj_file,
    };

    use super::*;

    #[test]
    fn test_compute_corners() {
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(1.0, 1.0, 1.0));

        let corners = bounds.corners();

        assert!(corners[0].distance_to_coord(0.0, 0.0, 0.0) < 0.001);
        assert!(corners[1].distance_to_coord(1.0, 0.0, 0.0) < 0.001);
        assert!(corners[2].distance_to_coord(1.0, 1.0, 0.0) < 0.001);
        assert!(corners[3].distance_to_coord(0.0, 1.0, 0.0) < 0.001);
        assert!(corners[4].distance_to_coord(0.0, 0.0, 1.0) < 0.001);
        assert!(corners[5].distance_to_coord(1.0, 0.0, 1.0) < 0.001);
        assert!(corners[6].distance_to_coord(1.0, 1.0, 1.0) < 0.001);
        assert!(corners[7].distance_to_coord(0.0, 1.0, 1.0) < 0.001);
    }

    #[test]
    fn test_compute_corners_non_origin() {
        let bounds = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));

        let corners = bounds.corners();

        assert!(corners[0].distance_to_coord(1.0, 1.0, 1.0) < 0.001);
        assert!(corners[1].distance_to_coord(2.0, 1.0, 1.0) < 0.001);
        assert!(corners[2].distance_to_coord(2.0, 2.0, 1.0) < 0.001);
        assert!(corners[3].distance_to_coord(1.0, 2.0, 1.0) < 0.001);
        assert!(corners[4].distance_to_coord(1.0, 1.0, 2.0) < 0.001);
        assert!(corners[5].distance_to_coord(2.0, 1.0, 2.0) < 0.001);
        assert!(corners[6].distance_to_coord(2.0, 2.0, 2.0) < 0.001);
        assert!(corners[7].distance_to_coord(1.0, 2.0, 2.0) < 0.001);
    }

    #[test]
    fn test_compute_wireframe() {
        let bounds = BoundingBox::new(Vec3::origin(), Vec3::new(1.0, 1.0, 1.0));

        let wireframe = bounds.as_wireframe();

        for line in wireframe {
            assert!(line.length() - 1.0 < 0.001);
        }
    }

    #[test]
    fn test_compute_wireframe_non_origin() {
        let bounds = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));

        let wireframe = bounds.as_wireframe();

        for line in wireframe {
            assert!(line.length() - 1.0 < 0.001);
        }
    }

    #[test]
    fn test_intersects_triangle() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );

        let bounds = BoundingBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert!(bounds.intersects(&triangle.bounds()));
    }

    #[test]
    fn test_create_from_objects() {
        let mesh: Mesh<f32> = parse_obj_file("assets/geometry/sphere.obj", false, false).unwrap();

        let bounds = mesh.bounds();

        let union = BoundingBox::from_objects(&mesh.as_triangles());

        assert!(
            bounds.min.distance_to_vec3(&union.min) < f32::epsilon(),
            "Incorrect min value. Mesh bounds was {} and triangles union was {}",
            bounds.min,
            union.min
        );

        assert!(
            bounds.max.distance_to_vec3(&union.max) < f32::epsilon(),
            "Incorrect max value. Mesh bounds was {} and triangles union was {}",
            bounds.max,
            union.max
        );
    }

    #[test]
    fn test_get_assigns_params() {
        let mut aabb = BoundingBox::new(Vec3::new(1., 1., 1.), Vec3::new(10., 10., 10.));

        let parameter_names: Vec<&str> = aabb.parameters().iter().map(|p| p.name).collect();

        for &param_name in parameter_names.iter() {
            aabb.set_parameter(param_name, Data::Vec3(Vec3::origin()));
        }

        assert!(
            aabb.min.distance_to_vec3(&Vec3::origin()).abs() < f64::epsilon(),
            "Expected param to be {} but was {}",
            Vec3::<f64>::origin(),
            aabb.min
        );
        assert!(
            aabb.max.distance_to_vec3(&Vec3::origin()).abs() < f64::epsilon(),
            "Expected param to be {}, but was {}",
            Vec3::<f64>::origin(),
            aabb.max
        );
    }
}
