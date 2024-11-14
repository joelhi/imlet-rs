use std::{error::Error, fmt::Debug};

use log::{error, info};
use num_traits::Float;

use crate::{
    types::{
        computation::{
            components::{Data, DataType, Parameter},
            traits::ImplicitFunction,
        },
        geometry::{Octree, Transform, Triangle, Vec3},
    },
    utils::io::parse_obj_file,
};

const MAX_LEAF_TRIANGLE_COUNT: usize = 12;
const MAX_TREE_DEPTH: u32 = 10;

/// Distance function from a mesh loaded from a file.
#[derive(Debug)]
pub struct MeshFile<T> {
    /// Storing the origin of the file.
    pub file_path: Option<String>,
    /// Option to center the geometry
    pub center: bool,
    /// Geometry to use for signed distance computation
    pub geometry_data: Option<Octree<Triangle<T>, T>>,
}

impl<T: Float + Send + Sync> MeshFile<T> {
    /// Create a new unset reference to some mesh data in a file.
    pub fn new() -> Self {
        Self {
            file_path: None,
            center: false,
            geometry_data: None,
        }
    }

    /// Create a new custom sdf from a geometry that implements the SignedDistance trait.
    ///
    /// # Arguments
    ///
    /// * `geometry` - Geomtry to use as base for signed distance computation.
    pub fn from_path(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mesh = parse_obj_file(file_path, false)?;
        let octree = mesh.compute_octree(MAX_TREE_DEPTH, MAX_LEAF_TRIANGLE_COUNT);
        Ok(Self {
            file_path: Some(file_path.to_string()),
            center: false,
            geometry_data: Some(octree),
        })
    }

    pub fn set_mesh_from_file(&mut self, file_path: &str) {
        let parse_result = parse_obj_file::<T>(&file_path, false);

        match parse_result {
            Ok(mut mesh) => {
                if self.center {
                    let translation = mesh.centroid() * -T::one();
                    mesh.transform_self_par(Transform::translation(translation));
                }

                let bounds = mesh.bounds();
                info!(
                    "Loaded mesh with bounds [{:?},{:?}]",
                    bounds.min.convert::<f32>(),
                    bounds.max.convert::<f32>()
                );
                let octree = mesh.compute_octree(MAX_TREE_DEPTH, MAX_LEAF_TRIANGLE_COUNT);
                self.geometry_data = Some(octree);
                self.file_path = Some(file_path.to_string());
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    }
}

static MESH_FILE_PARAMETERS: &[Parameter] = &[
    Parameter {
        name: "File Path",
        data_type: DataType::Text,
    },
    Parameter {
        name: "Center Geometry",
        data_type: DataType::Boolean,
    },
];

impl<T: Float + Send + Sync> ImplicitFunction<T> for MeshFile<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        if let Some(geometry_data) = &self.geometry_data {
            geometry_data.signed_distance(&Vec3::new(x, y, z))
        } else {
            T::zero()
        }
    }

    fn parameters(&self) -> &[Parameter] {
        MESH_FILE_PARAMETERS
    }

    fn set_parameter(&mut self, parameter_name: &str, data: Data<T>) {
        let mut new_file = String::new();
        if Parameter::set_text_from_param(parameter_name, &data, "File Path", &mut new_file) {
            self.set_mesh_from_file(&new_file);
        } else if Parameter::set_bool_from_param(
            parameter_name,
            &data,
            "Center Geometry",
            &mut self.center,
        ) {
            if let Some(file_path) = self.file_path.clone() {
                self.set_mesh_from_file(&file_path);
            }
        }
    }

    fn read_parameter(&self, parameter_name: &str) -> Option<Data<T>> {
        match parameter_name {
            "File Path" => {
                if let Some(file_path) = &self.file_path {
                    Some(Data::File(file_path.clone()))
                } else {
                    Some(Data::File("No file set.".to_string()))
                }
            }
            "Center Geometry" => Some(Data::<T>::Boolean(self.center)),
            _ => None,
        }
    }

    fn function_name(&self) -> &'static str {
        "ObjFile"
    }
}
