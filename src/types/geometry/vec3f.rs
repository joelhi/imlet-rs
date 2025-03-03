use std::{
    fmt::{self, Debug, Display},
    ops,
};

use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::Transform;

/// Vector or Point with 3 coordinates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    /// Create a new Vec3 from coordinates.
    /// # Arguments
    ///
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Float> Vec3<T> {
    /// Construct a new point at {0,0,0}
    pub fn origin() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Create a unit X-axis.
    pub fn x_axis() -> Vec3<T> {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Create a unit Y-axis.
    pub fn y_axis() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    /// Create a unit Z-axis.
    pub fn z_axis() -> Vec3<T> {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    /// Compute the minium x, y and z coordinates compared to another point.
    ///
    /// # Arguments
    /// * `pt` - Other point to compare coordinates to.
    pub fn min(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    /// Compute the maximum x, y and z coordinates compared to another point.
    ///
    /// # Arguments
    /// * `pt` - Other point to compare coordinates to.
    #[inline(always)]
    pub fn max(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    /// Compute the euclidian distance to another Vec3.
    ///
    /// # Arguments
    /// * `pt` - Other point to compute distance to.
    #[inline(always)]
    pub fn distance_to_vec3(&self, pt: &Vec3<T>) -> T {
        self.distance_to_vec3_squared(pt).sqrt()
    }

    /// Compute the euclidian distance to a location defined by x, y and z coordinates.
    ///
    /// # Arguments
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    #[inline(always)]
    pub fn distance_to_coord(&self, x: T, y: T, z: T) -> T {
        self.distance_to_coord_squared(x, y, z).sqrt()
    }

    /// Compute the euclidian squared distance to another Vec3.
    ///
    /// # Arguments
    /// * `pt` - Other point to compute distance to.
    #[inline(always)]
    pub fn distance_to_vec3_squared(&self, pt: &Vec3<T>) -> T {
        self.distance_to_coord_squared(pt.x, pt.y, pt.z)
    }

    /// Compute the euclidian squared distance to a location defined by x, y and z coordinates.
    ///
    /// # Arguments
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    #[inline(always)]
    pub fn distance_to_coord_squared(&self, x: T, y: T, z: T) -> T {
        (self.x - x).powi(2) + (self.y - y).powi(2) + (self.z - z).powi(2)
    }

    /// Computes a linear interpolaton between two Vec3 values.
    ///
    /// # Arguments
    /// * `start` - Vec to interpolate from.
    /// * `end` - Vec to interpolate to.
    /// * `t` - Parameter value, clamped between [0, 1].
    #[inline(always)]
    pub fn interpolate(start: &Vec3<T>, end: &Vec3<T>, t: T) -> Vec3<T> {
        let clamped = t.clamp(T::zero(), T::one());
        Self {
            x: start.x + clamped * (end.x - start.x),
            y: start.y + clamped * (end.y - start.y),
            z: start.z + clamped * (end.z - start.z),
        }
    }

    /// Computes the dot product between two Vec3 values.
    ///
    /// (x_1 * x_2) + (y_1 * y_2) + (z_1 * z_2)
    ///
    /// # Arguments
    /// * `rhs` - Vec to compute dot product with.
    #[inline(always)]
    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Computes the dot product between a Vec3 and a vector defined by three coordinates.
    ///
    /// (x_1 * x) + (y_1 * y) + (z_1 * z)
    ///
    /// # Arguments
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    #[inline(always)]
    pub fn dot_coord(&self, x: T, y: T, z: T) -> T {
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    /// Computes the cross product between two Vec3 values.
    /// # Arguments
    ///
    /// * `rhs` - Vec to compute cross product with.
    #[inline(always)]
    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Computes the angle between two Vec3 values.
    /// # Arguments
    ///
    /// * `rhs` - Vec to compute angle with.
    pub fn angle(&self, rhs: &Vec3<T>) -> Option<T> {
        let dot = self.dot(rhs);
        let len_self = self.magnitude();
        let len_other = rhs.magnitude();
        if len_self.is_zero() || len_other.is_zero() {
            None
        } else {
            let cosine = (dot / (len_self * len_other)).clamp(-T::one(), T::one());
            Some(cosine.acos())
        }
    }

    /// Compute the total length of a vector (distance to origin).
    #[inline(always)]
    pub fn magnitude(&self) -> T {
        self.distance_to_coord(T::zero(), T::zero(), T::zero())
    }

    /// Scale the magnitude of a vector with a scalar value.
    /// # Arguments
    ///
    /// * `scalar` - Scale factor.
    #[inline(always)]
    pub fn scale(self, scalar: T) -> Vec3<T> {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Normalize the vector, giving it a unit length.
    #[inline(always)]
    pub fn normalize(&self) -> Vec3<T> {
        *self * (T::one() / self.magnitude())
    }

    /// Compute the [Spherical Linear Interpolation](https://en.wikipedia.org/wiki/Slerp) of two vectors.
    ///
    /// *This performs a constant-speed motion along a unit-radius great circle arc, given the ends and an interpolation parameter between 0 and 1*
    /// # Arguments
    ///
    /// * `start` - Vec to interpolate from.
    /// * `end` - Vec to interpolate to.
    /// * `t` - Parameter value, clamped between [0, 1].
    pub fn slerp(start: Vec3<T>, end: Vec3<T>, t: T) -> Vec3<T> {
        let clamped = t.clamp(T::zero(), T::one());
        let start = start.normalize();
        let end = end.normalize();

        let dot = start.dot(&end).clamp(-T::one(), T::one());

        let theta = dot.acos();

        let sin_theta = theta.sin();

        if sin_theta == T::zero() {
            return start.scale(T::one() - clamped) + end.scale(clamped);
        }

        let a = ((T::one() - clamped) * theta).sin() / sin_theta;
        let b = (clamped * theta).sin() / sin_theta;

        start.scale(a) + end.scale(b)
    }

    /// Convert the internal data type to a new type *Q*. Returns [`None`] if the type conversion fails.
    ///
    /// Mainly to convert from [`f64`] to [`f32`] or to go from generic description to a concrete type.
    pub fn convert<Q: Float>(&self) -> Option<Vec3<Q>> {
        Some(Vec3::new(
            Q::from(self.x)?,
            Q::from(self.y)?,
            Q::from(self.z)?,
        ))
    }

    /// Apply a transformation to the Vec.
    pub fn transform(&self, transform: Transform<T>) -> Vec3<T> {
        let rotated = self.rotate(transform.rotation);
        rotated + transform.translation
    }

    pub(crate) fn rotate(&self, rotation: Vec3<T>) -> Vec3<T> {
        let (sin_x, cos_x) = (rotation.x.sin(), rotation.x.cos());
        let (sin_y, cos_y) = (rotation.y.sin(), rotation.y.cos());
        let (sin_z, cos_z) = (rotation.z.sin(), rotation.z.cos());

        let rotated_x = Vec3 {
            x: self.x,
            y: self.y * cos_x - self.z * sin_x,
            z: self.y * sin_x + self.z * cos_x,
        };

        let rotated_y = Vec3 {
            x: rotated_x.x * cos_y + rotated_x.z * sin_y,
            y: rotated_x.y,
            z: -rotated_x.x * sin_y + rotated_x.z * cos_y,
        };

        Vec3::new(
            rotated_y.x * cos_z - rotated_y.y * sin_z,
            rotated_y.x * sin_z + rotated_y.y * cos_z,
            rotated_y.z,
        )
    }

    /// Returns the default spatial tolerance value.
    pub fn default_tolerance() -> T {
        T::from(1E-5).expect("Failed to convert value of tolerance to target type T")
    }
}

impl<T: Float> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        {
            Self {
                x: self.x + _rhs.x,
                y: self.y + _rhs.y,
                z: self.z + _rhs.z,
            }
        }
    }
}

impl<T: Float> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Float> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = T;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.dot(&rhs)
    }
}

impl<T: Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_compute_angle_opposite() {
        let v1 = Vec3::new(1.392251041, 0.440162188, -0.14818595);

        let v2 = Vec3::new(-0.26339719, -0.08327343, 0.028035004);

        let angle = v1.angle(&v2).unwrap();
        assert!((angle - PI).abs() < 0.01);
    }

    #[test]
    fn test_serialize_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let json = serde_json::to_string_pretty(&v1).unwrap();
        let deserialized: Vec3<f64> = serde_json::from_str(&json).unwrap();

        assert!((v1.x - deserialized.x).abs() < 0.001);
        assert!((v1.y - deserialized.y).abs() < 0.001);
        assert!((v1.z - deserialized.z).abs() < 0.001);
    }

    #[test]
    fn test_slerp_opposite_vecs() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let interpolated = Vec3::slerp(v1, v2, 0.5);

        let expected_angle = PI / 4.0;
        let first_angle = interpolated.angle(&v1).unwrap();
        let second_angle = interpolated.angle(&v2).unwrap();
        assert!(
            (expected_angle - first_angle).abs() < 0.001,
            "Incorrect angle, expected {} but was {}",
            expected_angle,
            first_angle
        );
        assert!(
            (expected_angle - second_angle).abs() < 0.001,
            "Incorrect angle, expected {} but was {}",
            expected_angle,
            second_angle
        );
    }
}
