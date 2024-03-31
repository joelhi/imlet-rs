use super::XYZ;
use std::{collections::HashMap, usize};

const DEFAULT_SPATIAL_TOL: f32 = 1E-5;

pub struct SpatialHashGrid{
    map: HashMap<usize,Vec<usize>>,
    vertices: Vec<XYZ>,
    tolerance: f32
}

impl SpatialHashGrid{

    pub fn new()->Self{
        SpatialHashGrid{
            map: HashMap::new(),
            vertices: Vec::new(),
            tolerance: DEFAULT_SPATIAL_TOL
        }
    }

    pub fn new_with_tolerance(tolerance: f32)->Self{
        SpatialHashGrid{
            map: HashMap::new(),
            vertices: Vec::new(),
            tolerance: tolerance
        }
    }

    pub fn vertices(&self)->&Vec<XYZ>{
        &self.vertices
    }

    pub fn add_point(&mut self, v: XYZ)->usize{
        let hash = self.spatial_hash(v);
        match self.map.get(&hash).as_ref() {
            Some(&index) => {
                // Find closest point based on indices in list
                for &id in index{
                    if v.distance_to_xyz(self.vertices[id]) < self.tolerance{
                        return id;
                    }
                }
                // Add new entry and return current count
                self.get_new_id(hash, v)
            }
            None => {
                // Add new entry and return current count
                self.get_new_id(hash, v)
            }
        }
    }

    fn get_new_id(&mut self, hash: usize, v:XYZ)->usize{
        let id = self.vertices.len();
        self.map.insert(hash, vec![id]);
        self.vertices.push(v);
        id
    }

    pub fn spatial_hash(&self, v: XYZ)->usize{
        let multiplier = 1.0 / self.tolerance;
        let mut s_hash = 23;

        s_hash = s_hash * 37 + (v.x * multiplier) as usize;
        s_hash = s_hash * 37 + (v.y * multiplier) as usize;
        s_hash = s_hash * 37 + (v.z * multiplier) as usize;

        return s_hash;
    }
}



