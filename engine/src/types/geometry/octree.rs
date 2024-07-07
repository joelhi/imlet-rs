use std::{alloc::System, fmt::Debug};

use num_traits::Float;

use super::{BoundingBox, Triangle, Vec3};

const MAX_TRIANGLES: usize = 12;

#[derive(Debug)]
pub struct OctreeNode<T: Float + Debug> {
    pub bounds: BoundingBox<T>,
    pub triangles: Vec<Triangle<T>>,
    pub children: Option<Box<[Option<OctreeNode<T>>; 8]>>, // Use fixed-size array with Option
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

        let center = self.bounds.min + (self.bounds.max - self.bounds.min) * T::from(0.5).unwrap();
        let mut children: [Option<OctreeNode<T>>; 8] = Default::default(); // Initialize array with None

        for i in 0..8 {
            let offset = Vec3::<T>::new(
                if i & 1 == 0 { T::zero() } else { center.x },
                if i & 2 == 0 { T::zero() } else { center.y },
                if i & 4 == 0 { T::zero() } else { center.z },
            );

            let child_min = self.bounds.min + offset;
            let child_max = center + offset;
            let child_bounds = BoundingBox::new(child_min, child_max);

            let mut child_triangles = Vec::new();
            for triangle in &self.triangles {
                if triangle_intersects_aabb(triangle, &child_bounds) {
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

    pub fn closest_point(&self, point: &Vec3<T>, closest_so_far: &mut Option<(Vec3<T>, T)>) {
        println!("{}",self.triangles.len());
        if !self.bounds.contains(point) {
            return;
        }

        for triangle in &self.triangles {
            let closest_point_on_triangle = triangle.closest_pt(&point);
            let distance = (*point - closest_point_on_triangle).magnitude();

            if closest_so_far.is_none() || distance < closest_so_far.unwrap().1 {
                *closest_so_far = Some((closest_point_on_triangle, distance));
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                if let Some(child_node) = child {
                    child_node.closest_point(point, closest_so_far);
                }
            }
        }
    }
}

fn triangle_intersects_aabb<T: Float + Debug>(triangle: &Triangle<T>, aabb: &BoundingBox<T>) -> bool {
    triangle.bounds().intersects(aabb)
}

#[cfg(test)]
mod tests {
    use crate::types::geometry::Mesh;

    use super::*;

    #[test]
    fn test_build_octree() {
        let m: Mesh<f64> = crate::utils::io::parse_obj_file("../assets/geometry/sphere.obj").unwrap();

        
        let mut octree = OctreeNode::new(m.get_bounds(), m.as_triangles());
        octree.build(10, 15);

        // let mut closest_so_far = None;
        // octree.closest_point(&Vec3::origin(), &mut closest_so_far);

    }
}