use std::{fmt::Debug, fs, io::{self, Write}, path::Path};

use num_traits::Float;

use crate::types::geometry::Mesh;

pub fn mesh_to_obj<T: Float + Debug + Send + Sync>(mesh: &Mesh<T>) -> String {
    let mut data = String::new();

    for &v in mesh.get_vertices() {
        let v_string = format!("v {} {} {}\n", v.x.to_f32().expect("error"), v.y.to_f32().expect("error"), v.z.to_f32().expect("error"));
        data.push_str(&v_string);
    }

    for &f in mesh.get_faces() {
        let f_string = format!("f {} {} {}\n", f[0] + 1, f[1] + 1, f[2] + 1);
        data.push_str(&f_string);
    }

    data
}

pub fn write_as_obj<T: Float + Debug + Send + Sync>(mesh: &Mesh<T>, file_name: &str) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("obj");
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(mesh_to_obj(&mesh).as_bytes())?;
    
    Ok(())
}
