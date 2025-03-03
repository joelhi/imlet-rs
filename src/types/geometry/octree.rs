use log::debug;
use num_traits::Float;
use serde::{Deserialize, Serialize};

use super::{
    traits::{SignedDistance, SignedQuery, SpatialQuery},
    BoundingBox, Vec3,
};

type OctreeChildren<T> = Box<[Option<OctreeNode<T>>; 8]>;

/// Octree used for storing object and accelerating closest point and distance queries.
///
/// The octree can be built for any geometric object which implements the relevant traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Octree<Q, T> {
    objects: Vec<Q>,
    root: Option<OctreeNode<T>>,
    max_depth: u32,
    max_objects: usize,
}

impl<Q, T: Float> Octree<Q, T> {
    /// Create a new empty octree. To build the octree, add some objects and call [`Octree::build`].
    /// # Arguments
    ///
    /// * `max_depth` - Maximum allowed recursive depth when constructing the tree.
    /// * `max_objects` - Maximum number of objects per leaf node.
    pub fn new(max_depth: u32, max_objects: usize) -> Self {
        Self {
            objects: Vec::new(),
            root: None,
            max_depth,
            max_objects,
        }
    }

    /// Collect all the nested bounding boxes in the tree.
    ///
    /// # Returns
    ///
    /// * A list of all the bounding boxes.
    pub fn all_bounds(&self) -> Option<Vec<BoundingBox<T>>> {
        self.root.as_ref().map(|r| r.all_bounds())
    }

    /// Returns the full bounds of the octree if built.
    pub fn bounds(&self) -> Option<BoundingBox<T>> {
        self.root.as_ref().map(|r| r.bounds)
    }
}

impl<Q: SpatialQuery<T>, T: Float> Octree<Q, T> {
    /// Add objects to the octree. This method takes ownership of and returns self, and can be used in a builder-like pattern.
    ///
    /// * `objects` - The objects to add to the octree.
    pub fn add_objects(mut self, objects: &[Q]) -> Self {
        self.objects.extend_from_slice(objects);

        self
    }

    /// Build the octree from the objects.
    ///
    /// The method returns the built octree.
    pub fn build(mut self) -> Self {
        let mut node = OctreeNode::new(
            BoundingBox::from_objects(&self.objects).offset(T::from(0.1).unwrap()),
            (0..self.objects.len()).collect(),
        );
        node.build(self.max_depth, self.max_objects, &self.objects);

        self.root = Some(node);

        self
    }

    /// Compute the closest point in the octree to a query point.
    /// # Arguments
    ///
    /// * `query_point` - The point for which the closest point should be found.
    ///
    /// # Returns
    ///
    /// * A tuple with the closest point and the object on which it was found.
    pub fn closest_point(&self, query_point: &Vec3<T>) -> Option<(Vec3<T>, Q)> {
        self.root
            .as_ref()
            .map(|r| r.closest_point(query_point, &self.objects))
    }

    /// Collect all the objects in the tree withing a certain distance from a point.
    /// # Arguments
    ///
    /// * `query_point` - The point from which the search is carried out.
    /// * `search_distance` - The limit distance for object retrieval.
    ///
    /// # Returns
    ///
    /// * A list of all the objects with a closest point to the search point below the search distance.
    pub fn collect_nearby_objects(&self, query_point: &Vec3<T>, search_distance: T) -> Vec<Q> {
        let mut objects = Vec::new();
        let mut num_objects = 0;

        if let Some(root) = &self.root {
            root.collect_nearby_objects(
                query_point,
                search_distance,
                &mut objects,
                &mut num_objects,
                &self.objects,
            );
        }

        objects
    }
}

impl<Q: SignedQuery<T>, T: Float> Octree<Q, T> {
    pub fn signed_distance(&self, query_point: &Vec3<T>) -> T {
        if let Some(root) = &self.root {
            root.signed_distance(query_point, &self.objects)
        } else {
            log::warn!("Octree not built yet.");
            T::nan()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OctreeNode<T> {
    pub bounds: BoundingBox<T>,
    pub object_indices: Vec<usize>,
    pub children: Option<OctreeChildren<T>>,
}

impl<T: Float> OctreeNode<T> {
    pub fn new(bounds: BoundingBox<T>, object_indices: Vec<usize>) -> Self {
        Self {
            bounds,
            object_indices,
            children: None,
        }
    }

    pub fn all_bounds(&self) -> Vec<BoundingBox<T>> {
        let mut bounds = Vec::new();
        self.collect_bounds(&mut bounds);
        bounds
    }

    fn collect_bounds(&self, bounds: &mut Vec<BoundingBox<T>>) {
        bounds.push(self.bounds);

        if let Some(ref children) = self.children {
            for child in children.iter().flatten() {
                child.collect_bounds(bounds);
            }
        }
    }
}

impl<T: Float> OctreeNode<T> {
    pub fn build<Q: SpatialQuery<T>>(
        &mut self,
        max_depth: u32,
        max_triangles: usize,
        all_objects: &[Q],
    ) {
        if self.object_indices.len() <= max_triangles || max_depth == 0 {
            if max_depth == 0 && self.object_indices.len() > max_triangles {
                debug!(
                    "Reached max depth of octree with {} triangles. Please increase allowed depth.",
                    self.object_indices.len()
                );
            }
            debug!(
                "Built octree node with {} triangles and depth {}",
                self.object_indices.len(),
                max_depth
            );
            return;
        }

        let center = self.bounds.min + (self.bounds.max - self.bounds.min) * T::from(0.5).unwrap();
        let mut children: OctreeChildren<T> = Default::default();

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

            let mut child_indices = Vec::new();
            for &index in self.object_indices.iter() {
                if all_objects[index].bounds().intersects(&child_bounds) {
                    child_indices.push(index);
                }
            }

            if !child_indices.is_empty() {
                let mut child_node = OctreeNode::new(child_bounds, child_indices);
                child_node.build(max_depth - 1, max_triangles, all_objects);
                children[i] = Some(child_node);
            }
        }
        self.children = Some(children);
        self.object_indices.clear();
    }

    fn closest_point_recursive<Q: SpatialQuery<T>>(
        &self,
        point: &Vec3<T>,
        best_dist_sq: &mut T,
        best_point: &mut Vec3<T>,
        best_object: &mut Q,
        all_objects: &[Q],
    ) {
        for &index in &self.object_indices {
            let closest_point = all_objects[index].closest_point(point);
            let dist_sq = point.distance_to_vec3_squared(&closest_point);
            if dist_sq < *best_dist_sq {
                *best_dist_sq = dist_sq;
                *best_point = closest_point;
                *best_object = all_objects[index];
  
                if *best_dist_sq == T::zero() {
                    return;
                }
            }
        }
    
        if let Some(ref children) = self.children {
            let mut child_nodes: Vec<_> = children.iter().filter_map(|c| {
                c.as_ref().map(|child| {
                    let closest_dist_sq = child.bounds.closest_point(point).distance_to_vec3_squared(point);
                    (child, closest_dist_sq)
                })
            }).filter(|(_, d)| *d <= *best_dist_sq).collect();
    
            child_nodes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            for (child, closest_dist_sq) in child_nodes {
                if closest_dist_sq >= *best_dist_sq {
                    break;
                }
                child.closest_point_recursive(point, best_dist_sq, best_point, best_object, all_objects);
            }
        }
    }

    pub fn closest_point<Q: SpatialQuery<T>>(
        &self,
        point: &Vec3<T>,
        all_objects: &[Q],
    ) -> (Vec3<T>, Q) {
        let mut best_dist_sq = T::max_value();
        let mut best_point = *point;
        let mut best_object = SpatialQuery::default();
        self.closest_point_recursive(
            point,
            &mut best_dist_sq,
            &mut best_point,
            &mut best_object,
            all_objects,
        );
        (best_point, best_object)
    }

    pub fn collect_nearby_objects<Q: SpatialQuery<T>>(
        &self,
        point: &Vec3<T>,
        search_distance: T,
        objects: &mut Vec<Q>,
        num_objects: &mut usize,
        all_objects: &[Q],
    ) {
        if self.bounds.contains(point) {
            for &index in &self.object_indices {
                if all_objects[index]
                    .closest_point(point)
                    .distance_to_vec3(point)
                    < search_distance
                {
                    objects.push(all_objects[index]);
                    *num_objects += 1;
                }
            }

            if let Some(ref children) = self.children {
                for child in children.iter().filter_map(|c| c.as_ref()) {
                    child.collect_nearby_objects(
                        point,
                        search_distance,
                        objects,
                        num_objects,
                        all_objects,
                    );
                }
            }
        }
    }
}

impl<T: Float> OctreeNode<T> {
    pub fn signed_distance<Q: SignedQuery<T>>(&self, point: &Vec3<T>, all_objects: &[Q]) -> T {
        let (closest_point, closest_obj) = self.closest_point(point, all_objects);

        let direction = *point - closest_point;
        let normal = closest_obj.closest_point_with_normal(&closest_point).1;
        if normal.dot(&direction) < T::zero() {
            return -direction.magnitude();
        }

        direction.magnitude()
    }
}

impl<Q: SignedQuery<T> + Send + Sync, T: Float + Send + Sync> SignedDistance<T> for Octree<Q, T> {
    fn signed_distance(&self, x: T, y: T, z: T) -> T {
        self.signed_distance(&Vec3::new(x, y, z))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        types::geometry::Mesh,
        utils::{self, io::parse_obj_file},
    };

    use super::*;

    #[test]
    fn test_build_octree() {
        utils::logging::init_info();

        let m: Mesh<f64> = parse_obj_file("assets/geometry/sphere.obj", false, false).unwrap();
        let octree = m.compute_octree(10, 12);
        let bounds = octree.all_bounds().unwrap();

        assert!(
            bounds.len() == 183,
            "Incorrect number of bounds. Was {} but expected {}",
            bounds.len(),
            183
        );
    }

    #[test]
    fn test_closest_pt() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/sphere.obj", false, false).unwrap();

        let octree = m.compute_octree(10, 9);

        let (closest_pt, closest_tri) = octree.closest_point(&Vec3::origin()).unwrap();

        assert!(closest_pt.distance_to_coord(70.67, 97.26, 58.26) < 0.1);

        assert!(closest_tri.p1().distance_to_coord(59.12, 107.93, 54.46) < 0.1);
        assert!(closest_tri.p2().distance_to_coord(79.35, 93.23, 54.46) < 0.1);
        assert!(closest_tri.p3().distance_to_coord(53.38, 103.75, 68.40) < 0.1);
    }

    #[test]
    fn test_compute_signed_distance_box() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/box.obj", false, false).unwrap();
        let octree = m.compute_octree(10, 9);

        // Inside box
        let signed_distance = octree.signed_distance(&m.centroid());
        assert!((signed_distance + 10.0).abs() < 0.001);

        // Outside box
        let signed_distance = octree.signed_distance(&Vec3::new(30.0, 10.0, 10.0));
        assert!((signed_distance - 10.0).abs() < 0.001);

        // On corner
        let signed_distance = octree.signed_distance(&Vec3::new(20.0, 20.0, 20.0));
        assert!(signed_distance.abs() < 0.001);
    }

    #[test]
    fn test_compute_signed_distance_sphere() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/sphere.obj", false, false).unwrap();

        let octree = m.compute_octree(10, 9);

        // Inside sphere
        let signed_distance = octree.signed_distance(&m.centroid());
        assert!((signed_distance + 47.022).abs() < 0.001);

        // Outside sphere,
        let signed_distance = octree.signed_distance(&m.bounds().max);
        assert!((signed_distance - 35.896).abs() < 0.001);

        let (dx, _, _) = m.bounds().dimensions();
        // On corner
        let signed_distance =
            octree.signed_distance(&(m.centroid() + Vec3::new(dx / 2.0, 0.0, 0.0)));
        assert!(signed_distance.abs() < 0.001);
    }

    #[test]
    fn test_compute_signed_distance_bunny_leaking() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();

        let octree = m.compute_octree(10, 12);
        let query_point = Vec3::new(6.0, 45.0, 56.0);
        // Around ear

        let (closest_pt, _) = octree.closest_point(&query_point).unwrap();
        let signed_distance = octree.signed_distance(&query_point);

        let expected_closest_point = Vec3::new(7.219, 42.749, 56.182);
        let expected_signed_distance = 2.567;

        assert!(
            (signed_distance - expected_signed_distance).abs() < 0.001,
            "Incorrect signed distance. Was {} but expected {}",
            signed_distance,
            expected_signed_distance
        );
        assert!(
            closest_pt.distance_to_vec3(&expected_closest_point).abs() < 0.001,
            "Incorrect closest point. Was {} but expected {}",
            closest_pt,
            expected_closest_point
        );
    }

    #[test]
    fn test_compute_signed_distance_bunny_leaking_f32() {
        let m: Mesh<f32> = parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();

        let octree = m.compute_octree(10, 12);
        let query_point = Vec3::new(6.0, 45.0, 56.0);
        // Around ear

        let (closest_pt, _) = octree.closest_point(&query_point).unwrap();
        let signed_distance = octree.signed_distance(&query_point);

        let expected_closest_point = Vec3::new(7.219, 42.749, 56.182);
        let expected_signed_distance = 2.567;

        assert!(
            (signed_distance - expected_signed_distance).abs() < 0.001,
            "Incorrect signed distance. Was {} but expected {}",
            signed_distance,
            expected_signed_distance
        );
        assert!(
            closest_pt.distance_to_vec3(&expected_closest_point).abs() < 0.001,
            "Incorrect closest point. Was {} but expected {}",
            closest_pt,
            expected_closest_point
        );
    }

    #[test]
    fn test_compute_signed_distance_cow_leaking_edge() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/cow.obj", true, false).unwrap();

        let octree = m.compute_octree(10, 12);
        let query_point = Vec3::new(3.754165, -1.501405, 2.1629639);
        // Around ear

        let (closest_pt, _) = octree.closest_point(&query_point).unwrap();
        let signed_distance = octree.signed_distance(&query_point);

        let expected_closest_point = Vec3::new(4.070, -1.451, 2.233);
        let expected_signed_distance = 0.328;

        assert!(
            (signed_distance - expected_signed_distance).abs() < 0.001,
            "Incorrect signed distance. Was {} but expected {}",
            signed_distance,
            expected_signed_distance
        );
        assert!(
            closest_pt.distance_to_vec3(&expected_closest_point).abs() < 0.001,
            "Incorrect closest point. Was {} but expected {}",
            closest_pt,
            expected_closest_point
        );
    }

    #[test]
    fn test_compute_signed_distance_cow_leaking_edge_2() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/cow.obj", true, false).unwrap();

        let octree = m.compute_octree(10, 12);
        let query_point = Vec3::new(3.5741649, -1.581405, 2.062964);

        let (closest_pt, _) = octree.closest_point(&query_point).unwrap();

        let signed_distance = octree.signed_distance(&query_point);

        let expected_closest_point = Vec3::new(4.0780463, -1.4302883, 2.1699197);
        let expected_signed_distance = 0.537;

        assert!(
            closest_pt.distance_to_vec3(&expected_closest_point).abs() < 0.001,
            "Incorrect closest point. Was {} but expected {}",
            closest_pt,
            expected_closest_point
        );
        assert!(
            (signed_distance - expected_signed_distance).abs() < 0.001,
            "Incorrect signed distance. Was {} but expected {}",
            signed_distance,
            expected_signed_distance
        );
    }

    #[test]
    fn test_compute_signed_distance_cow_leaking_vertex() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/cow.obj", true, false).unwrap();

        let octree = m.compute_octree(10, 12);
        let query_point = Vec3::new(3.714165, -1.621405, 2.042964);

        let (closest_pt, _) = octree.closest_point(&query_point).unwrap();

        let signed_distance = octree.signed_distance(&query_point);

        let expected_closest_point = Vec3::new(4.083, -1.451, 2.176);
        let expected_signed_distance = 0.427;

        assert!(
            closest_pt.distance_to_vec3(&expected_closest_point).abs() < 0.001,
            "Incorrect closest point. Was {} but expected {}",
            closest_pt,
            expected_closest_point
        );
        assert!(
            (signed_distance - expected_signed_distance).abs() < 0.001,
            "Incorrect signed distance. Was {} but expected {}",
            signed_distance,
            expected_signed_distance
        );
    }

    #[test]
    fn test_compute_signed_distance_sphere_leaking() {
        let m: Mesh<f64> = parse_obj_file("assets/geometry/sphere.obj", false, false).unwrap();

        let octree = m.compute_octree(10, 12);

        // Outside sphere,
        let signed_distance = octree.signed_distance(&Vec3::new(103.180, 167.482, 119.522));
        assert!(
            (signed_distance - 2.391).abs() < 0.001,
            "Incorrect signed distance, expected {}, but was {}",
            2.391,
            signed_distance
        );
    }
}
