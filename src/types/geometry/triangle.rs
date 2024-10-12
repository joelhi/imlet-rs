use num_traits::Float;
use std::fmt::Debug;
use std::fmt::{self, Display};

use crate::types::computation::traits::ImplicitFunction;

use super::traits::SignedDistance;
use super::{
    traits::{SignedQuery, SpatialQuery},
    BoundingBox, Vec3,
};

/// A single triangle with vertices in 3d space.
#[derive(Debug, Clone, Copy)]
pub struct Triangle<T> {
    /// Positions of the three vertices.
    pub p: [Vec3<T>; 3],

    /// Optional normals at each vertex.
    pub n: Option<[Vec3<T>; 3]>,
}

impl<T> Triangle<T> {
    /// Create a new Triangle from three vertices with no normals.
    /// # Arguments
    ///
    /// * `p1` - First vertex.
    /// * `p2` - Second vertex.
    /// * `p3` - Third vertex.
    ///
    pub fn new(p1: Vec3<T>, p2: Vec3<T>, p3: Vec3<T>) -> Self {
        Self {
            p: [p1, p2, p3],
            n: None,
        }
    }

    /// Create a new Triangle from three vertices with no normals.
    /// # Arguments
    ///
    /// * `p1` - First vertex.
    /// * `p2` - Second vertex.
    /// * `p3` - Third vertex.
    /// * `n` - Array with normals for each vertex if applicable.
    pub fn with_normals(p1: Vec3<T>, p2: Vec3<T>, p3: Vec3<T>, n: Option<[Vec3<T>; 3]>) -> Self {
        Self { p: [p1, p2, p3], n }
    }
}

impl<T: Float> Triangle<T> {
    /// Create a new Triangle with all vertices at the origin {0,0,0}.
    pub fn zero() -> Self {
        Self {
            p: [Vec3::origin(), Vec3::origin(), Vec3::origin()],
            n: None,
        }
    }

    /// Return a copy of the first vertex in the triangle.
    #[inline]
    pub fn p1(&self) -> Vec3<T> {
        self.p[0]
    }

    /// Return a copy of the second vertex in the triangle.
    #[inline]
    pub fn p2(&self) -> Vec3<T> {
        self.p[1]
    }

    /// Return a copy of the third vertex in the triangle.
    #[inline]
    pub fn p3(&self) -> Vec3<T> {
        self.p[2]
    }

    /// Compute the area of the triangle.
    pub fn compute_area(&self) -> T {
        let a = self.p1().distance_to_vec3(&self.p2());
        let b = self.p2().distance_to_vec3(&self.p3());
        let c = self.p3().distance_to_vec3(&self.p1());
        let s = (a + b + c) / T::from(2.0).expect("Failed to convert number to T");
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    /// Compute the bounding box of the triangle in global coordinates.
    pub fn bounds(&self) -> BoundingBox<T> {
        BoundingBox::new(
            Vec3::new(
                self.p1().x.min(self.p2().x).min(self.p3().x),
                self.p1().y.min(self.p2().y).min(self.p3().y),
                self.p1().z.min(self.p2().z).min(self.p3().z),
            ),
            Vec3::new(
                self.p1().x.max(self.p2().x).max(self.p3().x),
                self.p1().y.max(self.p2().y).max(self.p3().y),
                self.p1().z.max(self.p2().z).max(self.p3().z),
            ),
        )
    }

    /// Compute the normal to the triangle face plane.
    pub fn face_normal(&self) -> Vec3<T> {
        let v1 = self.p2() - self.p1();
        let v2 = self.p3() - self.p1();
        v1.cross(&v2).normalize()
    }

    /// Returns the vertex normals if present, otherwise the face normal applied at each vertex.
    pub fn vertex_normals(&self) -> [Vec3<T>; 3] {
        if let Some(normals) = self.n {
            normals
        } else {
            let normal = self.face_normal();
            [normal, normal, normal]
        }
    }

    /// Compute the closest point on the triangle and at what triangle feature it is located from.
    ///
    /// See [`TriangleFeature`] for info on the feature classification.
    /// # Arguments
    ///
    /// * `query_point` - Point to compute the closest point from.
    pub fn closest_point(&self, query_point: &Vec3<T>) -> (TriangleFeature, Vec3<T>) {
        let eps = T::from(1e-7).unwrap();
        let ab = self.p[1] - self.p[0];
        let ac = self.p[2] - self.p[0];
        let ap = *query_point - self.p[0];

        let d1 = ab.dot(&ap);
        let d2 = ac.dot(&ap);
        if d1 <= eps && d2 <= eps {
            return (TriangleFeature::VERTEX(0), self.p[0]);
        }

        let bp = *query_point - self.p[1];
        let d3 = ab.dot(&bp);
        let d4 = ac.dot(&bp);
        if d3 >= -eps && d4 <= d3 + eps {
            return (TriangleFeature::VERTEX(1), self.p[1]);
        }

        let vc = d1 * d4 - d3 * d2;
        if vc <= eps && d1 >= -eps && d3 <= eps {
            let v = d1 / (d1 - d3);
            return (TriangleFeature::EDGE([0, 1]), self.p[0] + ab * v);
        }

        let cp = *query_point - self.p[2];
        let d5 = ab.dot(&cp);
        let d6 = ac.dot(&cp);
        if d6 >= -eps && d5 <= d6 + eps {
            return (TriangleFeature::VERTEX(2), self.p[2]);
        }

        let vb = d5 * d2 - d1 * d6;
        if vb <= eps && d2 >= -eps && d6 <= eps {
            let w = d2 / (d2 - d6);
            return (TriangleFeature::EDGE([0, 2]), self.p[0] + ac * w);
        }

        let va = d3 * d6 - d5 * d4;
        if va <= eps && (d4 - d3) >= -eps && (d5 - d6) >= -eps {
            let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
            return (
                TriangleFeature::EDGE([1, 2]),
                self.p[1] + (self.p[2] - self.p[1]) * w,
            );
        }

        let denom = T::one() / (va + vb + vc);
        let v = vb * denom;
        let w = vc * denom;
        (TriangleFeature::FACE, self.p[0] + ab * v + ac * w)
    }
}

impl<T: Display> fmt::Display for Triangle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "T: {}, {}, {}", self.p[0], self.p[1], self.p[2])
    }
}

impl<T: Float> SpatialQuery<T> for Triangle<T> {
    fn bounds(&self) -> BoundingBox<T> {
        self.bounds()
    }

    fn default() -> Self {
        Triangle::zero()
    }

    fn closest_point(&self, query_point: &Vec3<T>) -> Vec3<T> {
        self.closest_point(query_point).1
    }
}

impl<T: Float> SignedQuery<T> for Triangle<T> {
    fn closest_point_with_normal(&self, query_point: &Vec3<T>) -> (Vec3<T>, Vec3<T>) {
        let (closest_feature, closest_point) = self.closest_point(query_point);

        match closest_feature {
            TriangleFeature::VERTEX(i) => {
                let normals = self.vertex_normals();
                (closest_point, normals[i])
            }
            TriangleFeature::EDGE(e) => {
                let t = closest_point.distance_to_vec3(&self.p[e[0]])
                    / self.p[e[0]].distance_to_vec3(&self.p[e[1]]);
                let normals = self.vertex_normals();

                (closest_point, Vec3::slerp(normals[e[0]], normals[e[1]], t))
            }
            TriangleFeature::FACE => (closest_point, self.face_normal()),
        }
    }
}

impl<T: Float + Send + Sync> SignedDistance<T> for Triangle<T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        let query_point = Vec3::new(x, y, z);
        let (closest_point, normal) = self.closest_point_with_normal(&query_point);
        let qp = query_point - closest_point;

        let distance = query_point.distance_to_vec3(&query_point);
        if qp.dot(&normal) < T::zero() {
            return -distance;
        }
        distance
    }
}

impl<T: Float + Send + Sync> ImplicitFunction<T> for Triangle<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(x, y, z)
    }
}

/// Describes a feature of a triangle. Mainly used for classifying closest point look-ups.
pub enum TriangleFeature {
    /// Feature is a vertex in the triangle with specific index.
    VERTEX(usize),
    /// Feature is an edge in the triangle with index of edge points.
    EDGE([usize; 2]),
    /// Feature is on the face of the triangle (within the bounds).
    FACE,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_closest_point_on_face() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(1.0, 1.0, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::FACE));
        assert!(closest_point.distance_to_coord(1.0, 1.0, 0.0).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_edge_1() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(-2.5, 2.5, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::EDGE(_)));
        assert!(closest_point.distance_to_coord(0.0, 2.5, 0.0).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_edge_2() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(2.5, -2.5, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::EDGE(_)));
        assert!(closest_point.distance_to_coord(2.5, 0.0, 0.0).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_edge_3() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(5.0, 5.0, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::EDGE(_)));
        assert!(closest_point.distance_to_coord(2.5, 2.5, 0.0).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_v_1() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(-1.0, -1.0, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::VERTEX(_)));
        assert!(closest_point.distance_to_vec3(&v1).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_v_2() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(6.0, 0.0, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::VERTEX(_)));
        assert!(closest_point.distance_to_vec3(&v2).abs() < f64::epsilon());
    }

    #[test]
    fn test_closest_point_on_v_3() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(5.0, 0.0, 0.0);
        let v3 = Vec3::new(0.0, 5.0, 0.0);

        let tri = Triangle::new(v1, v2, v3);

        let test_point = Vec3::new(0.0, 6.0, 1.0);

        let (feature, closest_point) = tri.closest_point(&test_point);

        assert!(matches!(feature, TriangleFeature::VERTEX(_)));
        assert!(closest_point.distance_to_vec3(&v3).abs() < f64::epsilon());
    }
}
