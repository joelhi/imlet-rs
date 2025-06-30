use std::{error::Error, fmt::Debug};

use log::{error, info};
use num_traits::Float;
use serde::{Deserialize, Serialize};

use crate::{
    types::{
        computation::{
            model::{Data, DataType, Parameter},
            traits::{ImplicitComponent, ImplicitFunction},
        },
        geometry::{BoundingBox, Octree, Transform, Triangle, Vec3},
    },
    utils::io::parse_obj_file,
};

const MAX_LEAF_TRIANGLE_COUNT: usize = 24;
const MAX_TREE_DEPTH: usize = 12;

/// Distance function from a mesh loaded from a file.
///
/// # Example
///
/// ```rust
/// use imlet::types::computation::{
///     functions::MeshFile,
///     model::ImplicitModel,
///     operations::shape::{BooleanIntersection, Thickness},
/// };
///
/// // Create mesh file
/// let mesh_file = MeshFile::<f64>::from_path("assets/geometry/bunny.obj").unwrap();
/// let bounds = mesh_file.bounds().unwrap();
///
/// // Build model
/// let mut model = ImplicitModel::new();
/// let mesh_tag = model.add_function("Mesh", mesh_file).unwrap();
///
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshFile<T> {
    /// Storing the origin of the file.
    pub file_path: Option<String>,
    /// Option to center the geometry
    pub center: bool,
    /// Geometry to use for signed distance computation
    #[serde(skip_serializing)]
    pub geometry_data: Option<Octree<Triangle<T>, T>>,
}

impl<T: Float + Send + Sync> Default for MeshFile<T> {
    fn default() -> Self {
        Self::new()
    }
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

    /// Rebuild the internal octree based on the file path
    pub fn build(&mut self) {
        if let Some(file_path) = self.file_path.clone() {
            self.set_mesh_from_file(&file_path);
        }
    }

    /// Create a new custom sdf from a geometry that implements the SignedDistance trait.
    ///
    /// # Arguments
    ///
    /// * `geometry` - Geomtry to use as base for signed distance computation.
    pub fn from_path(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mesh = parse_obj_file(file_path, false, false)?;
        let octree = mesh.compute_octree(MAX_TREE_DEPTH, MAX_LEAF_TRIANGLE_COUNT);
        Ok(Self {
            file_path: Some(file_path.to_string()),
            center: false,
            geometry_data: Some(octree),
        })
    }

    pub fn set_mesh_from_file(&mut self, file_path: &str) {
        let parse_result = parse_obj_file::<T>(file_path, false, false);

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

    /// Return the bounds of the mesh in the file.
    pub fn bounds(&self) -> Option<BoundingBox<T>> {
        self.geometry_data.as_ref().and_then(|tree| tree.bounds())
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

impl<T: Float + Send + Sync + Serialize> ImplicitFunction<T> for MeshFile<T> {
    fn eval(&self, x: T, y: T, z: T) -> T {
        if let Some(geometry_data) = &self.geometry_data {
            geometry_data.signed_distance(&Vec3::new(x, y, z))
        } else {
            T::zero()
        }
    }
}

impl<T: Float + Send + Sync + Serialize> ImplicitComponent<T> for MeshFile<T> {
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

    fn name(&self) -> &'static str {
        "MeshFile"
    }
}
