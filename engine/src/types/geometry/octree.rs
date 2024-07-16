use std::fmt::Debug;

use num_traits::Float;

use super::{BoundingBox, Triangle, Vec3};

#[derive(Debug, Clone)]
pub struct OctreeNode<T: Float + Debug> {
    pub bounds: BoundingBox<T>,
    pub triangles: Vec<Triangle<T>>,
    pub children: Option<Box<[Option<OctreeNode<T>>; 8]>>,
}

impl<T: Float + Debug> OctreeNode<T> {
    pub fn new(bounds: BoundingBox<T>, triangles: Vec<Triangle<T>>) -> Self {
        Self {
            bounds: bounds,
            triangles: triangles,
            children: None,
        }
    }

    pub fn build(&mut self, max_depth: u32, max_triangles: usize) {
        if self.triangles.len() <= max_triangles || max_depth == 0 {
            return;
        }

        let center = self.bounds.min + (self.bounds.max - self.bounds.min) * T::from(0.5).unwrap();
        let mut children: [Option<OctreeNode<T>>; 8] = Default::default();

        for i in 0..8 {
            let offset = Vec3::<T>::new(
                if i & 1 == 0 {
                    T::zero()
                } else {
                    center.x - self.bounds.min.x
                },
                if i & 2 == 0 {
                    T::zero()
                } else {
                    center.y - self.bounds.min.y
                },
                if i & 4 == 0 {
                    T::zero()
                } else {
                    center.z - self.bounds.min.z
                },
            );

            let child_min = self.bounds.min + offset;
            let child_max = center + offset;
            let child_bounds = BoundingBox::new(child_min, child_max);

            let mut child_triangles = Vec::new();
            for triangle in self.triangles.iter() {
                if triangle.bounds().intersects(&child_bounds) {
                    child_triangles.push(*triangle);
                }
            }

            if !child_triangles.is_empty() {
                let mut child_node = OctreeNode::new(child_bounds, child_triangles);
                child_node.build(max_depth - 1, max_triangles);
                children[i] = Some(child_node);
            }
        }
        self.children = Some(Box::new(children));
        self.triangles.clear();
    }

    fn bounding_box_closer_than(&self, point: &Vec3<T>, dist_sq: T) -> bool {
        if self.bounds.contains(point) {
            return true;
        }

        let closest = self.bounds.closest_point(point);
        let closest_dist_sq = point.distance_to_vec3_squared(&closest);
        closest_dist_sq < dist_sq
    }

    fn closest_point_recursive(
        &self,
        point: &Vec3<T>,
        best_dist_sq: &mut T,
        best_point: &mut Vec3<T>,
        best_triangle: &mut Triangle<T>,
    ) {
        for triangle in &self.triangles {
            let closest_point = triangle.closest_pt(point);
            let dist_sq = point.distance_to_vec3_squared(&closest_point);
            if dist_sq < *best_dist_sq {
                *best_dist_sq = dist_sq;
                *best_point = closest_point;
                *best_triangle = *triangle;
            }
        }

        if let Some(ref children) = self.children {
            let mut child_nodes: Vec<&OctreeNode<T>> =
                children.iter().filter_map(|c| c.as_ref()).collect();

            child_nodes.sort_by(|a, b| {
                let a_dist = &a
                    .bounds
                    .closest_point(&point)
                    .distance_to_vec3_squared(point);
                let b_dist = &b
                    .bounds
                    .closest_point(&point)
                    .distance_to_vec3_squared(point);
                a_dist
                    .partial_cmp(&b_dist)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for child in child_nodes {
                if child.bounding_box_closer_than(point, *best_dist_sq) {
                    child.closest_point_recursive(point, best_dist_sq, best_point, best_triangle);
                }
            }
        }
    }

    pub fn closest_point(&self, point: &Vec3<T>) -> (Vec3<T>, Triangle<T>) {
        let mut best_dist_sq = T::max_value();
        let mut best_point = point.clone();
        let mut best_triangle = Triangle::zero();
        self.closest_point_recursive(
            point,
            &mut best_dist_sq,
            &mut best_point,
            &mut best_triangle,
        );
        (best_point, best_triangle)
    }

    pub fn get_all_bounds(&self) -> Vec<BoundingBox<T>> {
        let mut bounds = Vec::new();
        self.collect_bounds(&mut bounds);
        bounds
    }

    fn collect_bounds(&self, bounds: &mut Vec<BoundingBox<T>>) {
        bounds.push(self.bounds);

        if let Some(ref children) = self.children {
            for child in children.iter() {
                if let Some(ref child_node) = child {
                    child_node.collect_bounds(bounds);
                }
            }
        }
    }

    pub fn signed_distance(&self, point: Vec3<T>, use_pseudo_normal: bool) -> T {
        let (closest_point, closest_triangle) = self.closest_point(&point);

        let direction = point - closest_point;
        let normal = if use_pseudo_normal {
            self.angle_weighted_pseudonormal(&closest_point)
        } else {
            closest_triangle.normal()
        };

        let mut sign = T::one();
        if normal.dot(&direction) < T::zero() {
            sign = -sign;
        }

        sign * direction.magnitude()
    }

    fn gather_triangles_for_pseudonormal(
        &self,
        point: &Vec3<T>,
        triangles: &mut [Triangle<T>; 12],
        num_triangles: &mut usize,
    ) {
        if self.bounds.contains(point) {
            for t in &self.triangles {
                if t.closest_pt(point).distance_to_vec3(point) < T::from(0.01).unwrap() {
                    triangles[*num_triangles] = *t;
                    *num_triangles += 1;
                }
            }

            if let Some(ref children) = self.children {
                for child in children.iter().filter_map(|c| c.as_ref()) {
                    child.gather_triangles_for_pseudonormal(point, triangles, num_triangles);
                }
            }
        }
    }

    fn angle_weighted_pseudonormal(&self, point: &Vec3<T>) -> Vec3<T> {
        let mut triangles = [Triangle::zero(); 12];
        let mut num_triangles = 0;
        self.gather_triangles_for_pseudonormal(point, &mut triangles, &mut num_triangles);

        let mut pseudonormal = Vec3::origin();
        for index in 0..num_triangles {
            pseudonormal = pseudonormal + triangles[index].angle_weighted_normal(&point);
        }

        pseudonormal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        types::geometry::Mesh,
        utils::{self},
    };

    use super::*;

    #[test]
    fn test_build_octree() {
        utils::logging::init_info();
        let m: Mesh<f64> =
            crate::utils::io::parse_obj_file("../assets/geometry/bunny.obj").unwrap();

        let mut octree = OctreeNode::new(
            BoundingBox::new(Vec3::origin(), Vec3::new(10.0, 10.0, 10.0)),
            m.as_triangles(),
        );
        octree.build(10, 15);

        let (closest_pt, _) = octree.closest_point(&Vec3::origin());

        log::info!("{}", closest_pt);
    }
}
