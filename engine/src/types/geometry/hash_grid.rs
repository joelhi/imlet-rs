use num_traits::Float;

use super::Vec3;
use std::{collections::HashMap, fmt::Debug, usize};

const DEFAULT_SPATIAL_TOL: f32 = 1E-5;

/// Simple implementation of a spatial hash grid, not properly checking adjacent bins.
/// Tolerances may not be guaranteed to be satisfied in the event of close points in adjacent bins.
pub struct SpatialHashGrid<T> {
    map: HashMap<i64, Vec<usize>>,
    vertices: Vec<Vec3<T>>,
    tolerance: T,
}

impl<T: Float> SpatialHashGrid<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            vertices: Vec::new(),
            tolerance: T::from(DEFAULT_SPATIAL_TOL)
                .expect("Failed to convert default tolerance to T"),
        }
    }
    #[allow(dead_code)]
    pub fn with_tolerance(tolerance: T) -> Self {
        Self {
            map: HashMap::new(),
            vertices: Vec::new(),
            tolerance: tolerance,
        }
    }

    pub fn vertices(&self) -> &Vec<Vec3<T>> {
        &self.vertices
    }

    pub fn add_point(&mut self, v: Vec3<T>) -> usize {
        let hash = self.spatial_hash(v);
        match self.map.get_mut(&hash) {
            Some(index) => {
                // Find closest point based on indices in list
                for &id in index.iter() {
                    if v.distance_to_vec3(&self.vertices[id]) < self.tolerance {
                        return id;
                    }
                }
                // Add vertex to list and return current count
                let new_index = self.vertices.len();
                index.push(new_index);
                self.vertices.push(v);
                new_index
            }
            None => {
                // Add new entry and return current count
                self.new_id(hash, v)
            }
        }
    }

    fn new_id(&mut self, hash: i64, v: Vec3<T>) -> usize {
        let id = self.vertices.len();
        self.map.insert(hash, vec![id]);
        self.vertices.push(v);
        id
    }

    pub fn spatial_hash(&self, v: Vec3<T>) -> i64 {
        let multiplier = T::one() / self.tolerance;
        let mut s_hash: i64 = 23;

        s_hash = s_hash * 37 + (v.x * multiplier).to_i64().unwrap();
        s_hash = s_hash * 37 + (v.y * multiplier).to_i64().unwrap();
        s_hash = s_hash * 37 + (v.z * multiplier).to_i64().unwrap();

        return s_hash;
    }
}
