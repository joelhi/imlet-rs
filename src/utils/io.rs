use core::str;
use std::{
    fmt::Display,
    fs,
    io::{self, BufRead, Read, Write},
    path::Path,
};

use num_traits::Float;
use serde::{de::DeserializeOwned, Serialize};

use crate::types::{
    computation::{model::ImplicitModel, ScalarField},
    geometry::{Mesh, Vec3},
};

pub(crate) fn mesh_to_obj<T: Display>(mesh: &Mesh<T>) -> String {
    let mut data = String::new();

    for v in mesh.vertices().iter() {
        let v_string = format!("v {} {} {}\n", v.x, v.y, v.z);
        data.push_str(&v_string);
    }

    for f in mesh.faces() {
        let f_string = format!("f {} {} {}\n", f[0] + 1, f[1] + 1, f[2] + 1);
        data.push_str(&f_string);
    }

    if let Some(normals) = mesh.normals() {
        for n in normals.iter() {
            let v_string = format!("vn {} {} {}\n", n.x, n.y, n.z);
            data.push_str(&v_string);
        }
    }

    data
}

/// Write a mesh to an .obj file.
///
/// # Arguments
///
/// * `mesh` - Mesh to export.
/// * `file_name` - Name of the target file to be created, without .obj extension.
pub fn write_obj_file<T: Display>(mesh: &Mesh<T>, file_name: &str) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("obj");
    let mut file = fs::File::create(file_path)?;
    file.write_all(mesh_to_obj(mesh).as_bytes())?;

    log::info!(
        "Obj file with {} triangles and {} vertices written as {}",
        mesh.num_faces(),
        mesh.num_vertices(),
        file_name.to_owned() + ".obj"
    );

    Ok(())
}

use std::fs::File;

use super::math_helper::Pi;

/// Read a mesh from an .obj file.
///
/// # Arguments
///
/// * `file_path` - Relative path to the file.
/// * `flip_yz` - Option to flip the y and z directions. Imlet uses z as up-direction so if the mesh has y, you may want to flip it.
pub fn parse_obj_file<T: Float + Send + Sync>(
    file_path: &str,
    flip_yz: bool,
    read_normals: bool,
) -> Result<Mesh<T>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    let extension = path.extension().ok_or_else(|| {
        format!(
            "Cannot read file {}. Only .obj files are supported.",
            file_path
        )
    })?;

    if !extension.eq_ignore_ascii_case("obj") {
        return Err(format!(
            "Cannot read file {}. Only .obj files are supported.",
            file_path
        )
        .into());
    }

    let file = File::open(path)?;

    let mut vertices: Vec<Vec3<T>> = Vec::new();
    let mut faces: Vec<[usize; 3]> = Vec::new();
    let mut normals: Vec<Vec3<T>> = Vec::new();
    let mut mesh = Mesh::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                // Parse vertex position
                if parts.len() != 4 {
                    return Err("Invalid vertex format. Make sure file is triangulated.,".into());
                }
                let x: f32 = parts[1].parse()?;
                let y: f32 = parts[2].parse()?;
                let z: f32 = parts[3].parse()?;
                vertices.push(Vec3::new(
                    T::from(x).unwrap(),
                    if flip_yz {
                        -T::from(z).unwrap()
                    } else {
                        T::from(y).unwrap()
                    },
                    if flip_yz {
                        T::from(y).unwrap()
                    } else {
                        T::from(z).unwrap()
                    },
                ));
            }
            "f" => {
                // Parse face indices
                if parts.len() != 4 {
                    return Err("Invalid face format".into());
                }
                let mut face: [usize; 3] = [0; 3];
                for i in 0..3 {
                    let indices: Vec<&str> = parts[i + 1].split("/").collect();
                    let index: usize = indices[0].parse().unwrap();
                    face[i] = index - 1;
                }
                faces.push(face);
            }
            "n" => {
                if read_normals {
                    if parts.len() != 4 {
                        return Err(
                            "Invalid vertex format. Make sure file is triangulated.,".into()
                        );
                    }
                    let n_x: f32 = parts[1].parse()?;
                    let n_y: f32 = parts[2].parse()?;
                    let n_z: f32 = parts[3].parse()?;
                    normals.push(Vec3::new(
                        T::from(n_x).unwrap(),
                        T::from(n_y).unwrap(),
                        T::from(n_z).unwrap(),
                    ));
                }
            }
            _ => continue,
        }
    }

    mesh.add_vertices(&vertices);
    mesh.add_faces(&faces);
    if read_normals {
        mesh.set_normals(&normals);
    } else {
        mesh.compute_vertex_normals_par();
    }

    log::info!(
        "Obj file {} with {} vertices and {} faces successfully read.",
        file_path,
        mesh.num_vertices(),
        mesh.num_faces()
    );

    Ok(mesh)
}

/// Write a ScalarField to a .csv file.
///
/// This will create a csv with the columns *{x, y, z, v}* where
/// - `x` is the x cooridinate of the data point
/// - `y` is the y cooridinate of the data point
/// - `z` is the z cooridinate of the data point
/// - `v` is value of the data point
///
/// # Arguments
///
/// * `field` - Field to export.
/// * `file_name` - Name of the target file to be created, without .csv extension.
pub fn write_field_csv<T: Float + Display>(
    field: &ScalarField<T>,
    file_name: &str,
) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("csv");
    let mut file = fs::File::create(file_path)?;
    file.write_all(field_as_data(field).as_bytes())?;
    Ok(())
}

fn field_as_data<T: Float + Display>(field: &ScalarField<T>) -> String {
    let mut data = String::new();
    data.push_str("x, y, z, v\n");
    for (idx, v) in field.data().iter().enumerate() {
        let (i, j, k) = field.point_index3d(idx);
        let v_string = format!(
            "{},{},{},{}\n",
            field.origin().x
                + field.cell_size() * T::from(i).expect("Failed to convert number to T"),
            field.origin().y
                + field.cell_size() * T::from(j).expect("Failed to convert number to T"),
            field.origin().z
                + field.cell_size() * T::from(k).expect("Failed to convert number to T"),
            v
        );
        data.push_str(&v_string);
    }

    data
}

/// Write an imlet model to a text file as json.
pub fn write_model_to_file<T: Float + Send + Sync + Serialize + 'static + Pi>(
    model: &ImplicitModel<T>,
    file_name: &str,
) -> io::Result<()> {
    let json = serde_json::ser::to_string_pretty(&model)?;
    let file_path = Path::new(file_name).with_extension("json");
    let mut file = fs::File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    log::info!(
        "Model with {} componets successfully saved as {}.",
        model.all_components().len(),
        file_name.to_owned() + ".json"
    );

    Ok(())
}

/// Deserialize an imlet model from a json file.
///
/// # Arguments
///
/// * `file_name` - Name of the file to read with the `.json` extension.
///
/// # Returns
///
/// An error is something went wrong, such as if the file can't be found or the deserialization failed.
///
/// Returns Ok() with the model if the read was successful.
pub fn read_model_from_file<
    T: Float + Send + Sync + Serialize + 'static + Pi + DeserializeOwned,
>(
    file_path: &str,
) -> Result<ImplicitModel<T>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    let extension = path.extension().ok_or_else(|| {
        format!(
            "Cannot read file {}. No valid extension provided. Should be .json.",
            file_path
        )
    })?;

    if !extension.eq_ignore_ascii_case("json") {
        return Err(format!(
            "Cannot read file {}. Only .json files are supported.",
            file_path
        )
        .into());
    }

    let mut file = File::open(path)?;

    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    let deserialized: ImplicitModel<T> = serde_json::de::from_slice(&data)?;

    Ok(deserialized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simple_model() {
        let model: ImplicitModel<f32> =
            read_model_from_file("assets/models/gyroid_model.json").unwrap();

        let val = model.evaluate_at("Output", 0., 0., 0.).unwrap();

        let expected_val = 41.60254;
        assert!(
            (val - expected_val).abs() < f32::epsilon(),
            "Wrong value returned from model. Was {}, but should have been {}",
            val,
            expected_val
        );
    }

    #[test]
    fn test_deserialize_model_with_file() {
        let model: ImplicitModel<f32> =
            read_model_from_file("assets/models/bunny_model.json").unwrap();

        let val = model.evaluate_at("Output", 0., 0., 0.).unwrap();

        let expected_val = 13.442741;
        assert!(
            (val - expected_val).abs() < f32::epsilon(),
            "Wrong value returned from model. Was {}, but should have been {}",
            val,
            expected_val
        );
    }
}
