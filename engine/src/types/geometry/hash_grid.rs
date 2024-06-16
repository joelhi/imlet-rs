use num_traits::Float;

use super::Vec3;
use std::{collections::HashMap, fmt::Debug, usize};

const DEFAULT_SPATIAL_TOL: f32 = 1E-3;

const MAX_BIN_SIZE: usize = 4;

pub struct SpatialHashGrid<T: Float + Debug> {
    map: HashMap<usize, [Option<usize>; MAX_BIN_SIZE]>,
    vertices: Vec<Vec3<T>>,
    tolerance: T,
}

impl<T: Float + Debug> SpatialHashGrid<T> {
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
                let mut last_pos = Option::None;
                for (pos, &id) in index.iter().enumerate() {
                    match id {
                        Some(value) => {
                            if v.distance_to_vec3(&self.vertices[value]) < self.tolerance {
                                return value;
                            }
                            last_pos = Some(pos);
                        },
                        None => {
                            break;
                        },
                    }
                }
                let new_index = self.vertices.len();
                self.vertices.push(v);
                match last_pos {
                    Some(value) => index[value + 1] = Option::Some(new_index),
                    None => panic!("More than allowed max values in a bin reached. Please lower spatial tolerance to decrease bin size."),
                }

                return new_index;
                
            }
            None => {
                // Add new entry and return current count
                self.get_new_id(hash, v)
            }
        }
    }

    fn get_new_id(&mut self, hash: usize, v: Vec3<T>) -> usize {
        let id = self.vertices.len();
        let mut bin_ids = [Option::None; MAX_BIN_SIZE];
        bin_ids[0] = Some(id);
        self.map.insert(hash, bin_ids);
        self.vertices.push(v);
        id
    }

    pub fn spatial_hash(&self, v: Vec3<T>) -> usize {
        let multiplier = T::one() / self.tolerance;
        let mut s_hash = 23;

        s_hash = s_hash * 37
            + (v.x * multiplier)
                .to_usize()
                .expect("Failed to convert T to usize");
        s_hash = s_hash * 37
            + (v.y * multiplier)
                .to_usize()
                .expect("Failed to convert T to usize");
        s_hash = s_hash * 37
            + (v.z * multiplier)
                .to_usize()
                .expect("Failed to convert T to usize");

        return s_hash;
    }
}
