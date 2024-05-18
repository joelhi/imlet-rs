use std::{fs, io::{self, Write}, path::Path};

use crate::engine::types::geometry::Mesh;

pub fn mesh_to_obj(mesh: &Mesh) -> String {
    let mut data = String::new();

    for &v in mesh.get_vertices() {
        let v_string = format!("v {} {} {}\n", v.x, v.y, v.z);
        data.push_str(&v_string);
    }

    for &f in mesh.get_faces() {
        let f_string = format!("f {} {} {}\n", f[0] + 1, f[1] + 1, f[2] + 1);
        data.push_str(&f_string);
    }

    data
}

pub fn write_as_obj(mesh: &Mesh, file_name: &str) -> io::Result<()> {
    let file_path = Path::new(file_name).with_extension("obj");
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(mesh_to_obj(&mesh).as_bytes())?;
    
    Ok(())
}
