use std::{
    any,
    fmt::{self, Debug},
    ops,
};

use num_traits::Float;
use serde::{Deserialize, Serialize};

/// Vector or Point with 3 coordinates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3<T>
where
    T: Float + Debug,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float + Debug> Vec3<T> {
    /// Create a new Vec3 from coordinates.
    /// # Arguments
    ///
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }

    /// Compute the euclidian distance to another Vec3.
    ///
    /// # Arguments
    /// * `pt` - Other point to compute distance to.
    pub fn distance_to_vec3(&self, pt: &Vec3<T>) -> T {
        self.distance_to_vec3_squared(&pt).sqrt()
    }

    /// Compute the euclidian distance to a location defined by x, y and z coordinates.
    ///
    /// # Arguments
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    pub fn distance_to_coord(&self, x: T, y: T, z: T) -> T {
        self.distance_to_coord_squared(x, y, z).sqrt()
    }

    /// Compute the euclidian squared distance to another Vec3.
    ///
    /// # Arguments
    /// * `pt` - Other point to compute distance to.
    pub fn distance_to_vec3_squared(&self, pt: &Vec3<T>) -> T {
        self.distance_to_coord_squared(pt.x, pt.y, pt.z)
    }

    /// Compute the euclidian squared distance to a location defined by x, y and z coordinates.
    ///
    /// # Arguments
    /// * `x` - X coordinate.
    /// * `y` - Y coordinate.
    /// * `z` - Z coordinate.
    pub fn distance_to_coord_squared(&self, x: T, y: T, z: T) -> T {
        (self.x - x).powi(2) + (self.y - y).powi(2) + (self.z - z).powi(2)
    }

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

    /// Computes a linear interpolaton between two Vec3 values.
    ///
    /// # Arguments
    /// * `start` - Vec to interpolate from.
    /// * `end` - Vec to interpolate to.
    /// * `t` - Parameter value, clamped between [0, 1].
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
    pub fn dot_coord(&self, x: T, y: T, z: T) -> T {
        (self.x * x) + (self.y * y) + (self.z * z)
    }

    /// Computes the cross product between two Vec3 values.
    /// # Arguments
    ///
    /// * `rhs` - Vec to compute cross product with.
    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Computes the cangle between two Vec3 values.
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
    pub fn magnitude(&self) -> T {
        self.distance_to_coord(T::zero(), T::zero(), T::zero())
    }

    /// Scale the magnitude of a vector with a scalar value.
    /// # Arguments
    ///
    /// * `scalar` - Scale factor.
    pub fn scale(self, scalar: T) -> Vec3<T> {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Normalize the vector, giving it a unit length.
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

    /// Convert the internal data type to a new type *Q*.
    pub fn convert<Q: Float + Debug>(&self) -> Vec3<Q> {
        Vec3::new(
            Q::from(self.x).expect(&format!(
                "Failed to convert from {} to {}",
                any::type_name::<Q>(),
                any::type_name::<T>()
            )),
            Q::from(self.y).expect(&format!(
                "Failed to convert from {} to {}",
                any::type_name::<Q>(),
                any::type_name::<T>()
            )),
            Q::from(self.z).expect(&format!(
                "Failed to convert from {} to {}",
                any::type_name::<Q>(),
                any::type_name::<T>()
            )),
        )
    }

    /// Returns the default spatial tolerance value.
    pub fn default_tolerance() -> T {
        T::from(1E-7).expect("Fail")
    }
}

impl<T: Float + Debug> ops::Add<Vec3<T>> for Vec3<T> {
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

impl<T: Float + Debug> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Float + Debug> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float + Debug> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = T;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self.dot(&rhs)
    }
}

impl<T: Float + Debug> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{{}, {}, {}}}",
            self.x.to_f32().unwrap(),
            self.y.to_f32().unwrap(),
            self.z.to_f32().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_compute_angle_opposite() {
        let v1 = Vec3::new(
            1.3922510409397368,
            0.44016218835974374,
            -0.14818594990623979,
        );

        let v2 = Vec3::new(
            -0.26339719056661082,
            -0.083273404291623443,
            0.028035003558268268,
        );

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
