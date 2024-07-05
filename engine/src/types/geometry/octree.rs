use std::fmt::Debug;

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
                children[i] = Some(OctreeNode::new(child_bounds, child_triangles));
            }
        }

        for child in &mut children {
            if let Some(child_node) = child {
                child_node.build(max_depth - 1, max_triangles);
            }
        }

        self.children = Some(Box::new(children));
    }
}

fn triangle_intersects_aabb<T: Float + Debug>(triangle: &Triangle<T>, aabb: &BoundingBox<T>) -> bool {
    triangle.bounds().intersects(aabb)
}
