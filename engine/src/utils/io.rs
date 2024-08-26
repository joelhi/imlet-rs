use std::{
    fmt::Debug,
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

use num_traits::Float;

use crate::types::{
    computation::DenseField,
    geometry::{Mesh, Vec3},
};

pub fn mesh_to_obj<T: Float + Debug + Send + Sync>(mesh: &Mesh<T>) -> String {
    let mut data = String::new();

    for &v in mesh.vertices() {
        let v_string = format!(
            "v {} {} {}\n",
            v.x.to_f32().expect("error"),
            v.y.to_f32().expect("error"),
            v.z.to_f32().expect("error")
        );
        data.push_str(&v_string);
    }

    for &f in mesh.faces() {
        let f_string = format!("f {} {} {}\n", f[0] + 1, f[1] + 1, f[2] + 1);
        data.push_str(&f_string);
    }

    data
}

pub fn write_obj_file<T: Float + Debug + Send + Sync>(
    mesh: &Mesh<T>,
    file_name: &str,
) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("obj");

    let mut file = fs::File::create(file_path)?;
    file.write_all(mesh_to_obj(&mesh).as_bytes())?;
    Ok(())
}

use std::fs::File;

pub fn parse_obj_file<T: Float + Debug + Send + Sync>(
    file_path: &str,
) -> Result<Mesh<T>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;

    let mut vertices: Vec<Vec3<T>> = Vec::new();
    let mut faces: Vec<[usize; 3]> = Vec::new();
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
                    return Err("Invalid vertex format".into());
                }
                let x: f32 = parts[1].parse()?;
                let y: f32 = parts[2].parse()?;
                let z: f32 = parts[3].parse()?;
                vertices.push(Vec3::new(
                    T::from(x).unwrap(),
                    T::from(y).unwrap(),
                    T::from(z).unwrap(),
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
            _ => continue,
        }
    }

    mesh.add_vertices(&vertices);
    mesh.add_faces(&faces);
    mesh.compute_vertex_normals();

    log::info!(
        "Obj file with {} vertices and {} faces successfully read.",
        mesh.num_vertices(),
        mesh.num_faces()
    );

    Ok(mesh)
}

pub fn write_field_csv<T: Float + Debug + Send + Sync>(
    field: &DenseField<T>,
    file_name: &str,
) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("csv");
    let mut file = fs::File::create(file_path)?;
    file.write_all(get_field_as_data(&field).as_bytes())?;
    Ok(())
}

fn get_field_as_data<T: Float + Debug + Send + Sync>(field: &DenseField<T>) -> String {
    let mut data = String::new();
    data.push_str("x, y, z, v\n");
    for (idx, v) in field.data().iter().enumerate() {
        let (i, j, k) = field.get_point_index3d(idx);
        let v_string = format!(
            "{:?},{:?},{:?},{:?}\n",
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
