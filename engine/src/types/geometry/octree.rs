
use std::{collections::HashSet, fmt::Debug};

use num_traits::Float;

use super::{BoundingBox, Triangle, Vec3};

#[derive(Debug)]
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
        // Check if the current node needs to be subdivided
        if self.triangles.len() <= max_triangles || max_depth == 0 {
            return;
        }

        let mut t: HashSet<usize> = HashSet::new();

        let center = self.bounds.min + (self.bounds.max - self.bounds.min) * T::from(0.5).unwrap();
        let mut children: [Option<OctreeNode<T>>; 8] = Default::default(); // Initialize array with None

        for i in 0..8 {
            let offset = Vec3::<T>::new(
                if i & 1 == 0 { T::zero() } else { center.x - self.bounds.min.x },
                if i & 2 == 0 { T::zero() } else { center.y - self.bounds.min.y },
                if i & 4 == 0 { T::zero() } else { center.z - self.bounds.min.z },
            );

            let child_min = self.bounds.min + offset;
            let child_max = center + offset;
            let child_bounds = BoundingBox::new(child_min, child_max);

            let mut child_triangles = Vec::new();
            for (index, triangle) in self.triangles.iter().enumerate() {
                if triangle_intersects_aabb(triangle, &child_bounds) {
                    t.insert(index);
                    child_triangles.push(*triangle);
                }
            }

            if !child_triangles.is_empty() {
                let mut child_node = OctreeNode::new(child_bounds, child_triangles);
                child_node.build(max_depth - 1, max_triangles); // Recursively build child nodes
                children[i] = Some(child_node);
            }
        }

        // Replace the current node's triangles with the subdivided children
        self.children = Some(Box::new(children));
        self.triangles.clear(); // Clear the current node's triangles as they are now distributed among children
    }

    
    // Helper function to check if a bounding box might contain a point closer than a given distance
    fn bounding_box_closer_than(&self, point: &Vec3<T>, dist_sq: T) -> bool {
        let mut closest = T::zero();
        if point.x < self.bounds.min.x {
            closest = closest + (self.bounds.min.x - point.x).powi(2);
        } else if point.x > self.bounds.max.x {
            closest = closest + (point.x - self.bounds.max.x).powi(2);
        }
        if point.y < self.bounds.min.y {
            closest = closest + (self.bounds.min.y - point.y).powi(2);
        } else if point.y > self.bounds.max.y {
            closest = closest +(point.y - self.bounds.max.y).powi(2);
        }
        if point.z < self.bounds.min.z {
            closest = closest + (self.bounds.min.z - point.z).powi(2);
        } else if point.z > self.bounds.max.z {
            closest = closest + (point.z - self.bounds.max.z).powi(2);
        }
        closest < dist_sq
    }

    fn closest_point_recursive(
        &self,
        point: &Vec3<T>,
        best_dist_sq: &mut T,
        best_point: &mut Vec3<T>,
        best_triangle: &mut Triangle<T>
    ) {

        for triangle in &self.triangles {
            let closest_point = triangle.closest_pt(point);
            let dist_sq = point.distance_to_vec3(&closest_point);
            if dist_sq < *best_dist_sq {
                *best_dist_sq = dist_sq;
                *best_point = closest_point;
                *best_triangle = *triangle;
            }
        }

        // If the current node has children, recursively search in the children nodes
        if let Some(ref children) = self.children {
            let mut child_nodes: Vec<&OctreeNode<T>> = children.iter().filter_map(|c| c.as_ref()).collect();

            // Sort child nodes by the distance to the search point, prioritizing the node containing the search point
            child_nodes.sort_by(|a, b| {
                let a_dist = &a.bounds.closest_point(&point).distance_to_vec3(point);
                let b_dist = &b.bounds.closest_point(&point).distance_to_vec3(point);
                a_dist.partial_cmp(&b_dist).unwrap_or(std::cmp::Ordering::Equal)
            });

            for child in child_nodes {
                if child.bounding_box_closer_than(point, *best_dist_sq) {
                    child.closest_point_recursive(point, best_dist_sq, best_point, best_triangle);
                }
            }
        }
    }

    // Public function to find the closest point in the octree
    pub fn closest_point(&self, point: &Vec3<T>) -> (Vec3<T>, Triangle<T>) {
        let mut best_dist_sq = T::max_value();
        let mut best_point = point.clone();
        let mut best_triangle = Triangle::zero();
        self.closest_point_recursive(point, &mut best_dist_sq, &mut best_point, &mut best_triangle);
        (best_point, best_triangle)
    }

    pub fn get_all_objects(&self) -> Vec<Triangle<T>> {
        let mut objects = Vec::new();
        self.collect_objects(&mut objects);
        objects
    }

    fn collect_objects(&self, objects: &mut Vec<Triangle<T>>) {
        objects.extend(self.triangles.iter().cloned());
        if let Some(ref children) = self.children {
            for child in children.iter() {
                if let Some(ref child_node) = child {
                    child_node.collect_objects(objects);
                }
            }
        }
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

     // A method to compute the signed distance to the surface
     pub fn signed_distance(&self, point: Vec3<T>) -> T {
        let (closest_point, _) = self.closest_point(&point); // Implement this as described previously
        let pseudonormal = self.angle_weighted_pseudonormal(&closest_point); // Implement this method
        
        let direction = point - closest_point;
        let sign = if pseudonormal.dot(&direction) < T::zero() { -T::one() } else { T::one() };

        sign * direction.magnitude()
    }

    fn gather_triangles_for_pseudonormal(&self, point: &Vec3<T>, triangles: &mut Vec<Triangle<T>>) {
        if self.bounds.contains(point) {
            triangles.extend_from_slice(&self.triangles);
            
            if let Some(ref children) = self.children {
                for child in children.iter().filter_map(|c| c.as_ref()) {
                    child.gather_triangles_for_pseudonormal(point, triangles);
                }
            }
        }
    }

    // A method to calculate the angle-weighted pseudonormal
    fn angle_weighted_pseudonormal(&self, point: &Vec3<T>) -> Vec3<T> {
        let mut triangles = Vec::new();
        self.gather_triangles_for_pseudonormal(point, &mut triangles);

        let mut pseudonormal = Vec3::origin();
        for triangle in &triangles {
            pseudonormal = pseudonormal + triangle.angle_weighted_normal(*point);
        }

        pseudonormal.normalize()
    }
}

fn triangle_intersects_aabb<T: Float + Debug>(triangle: &Triangle<T>, aabb: &BoundingBox<T>) -> bool {
    triangle.bounds().intersects(aabb)
}

#[cfg(test)]
mod tests {
    use crate::{types::geometry::Mesh, utils::{self}};

    use super::*;

    #[test]
    fn test_build_octree() {

        utils::logging::init_info();
        let m: Mesh<f64> = crate::utils::io::parse_obj_file("../assets/geometry/bunny.obj").unwrap();

        let mut octree = OctreeNode::new(BoundingBox::new(Vec3::origin(), Vec3::new(10.0,10.0,10.0)), m.as_triangles());
        octree.build(10, 15);

        let (closest_pt, closest_tri) = octree.closest_point(&Vec3::origin());

        log::info!("{}", closest_pt);

    }
}