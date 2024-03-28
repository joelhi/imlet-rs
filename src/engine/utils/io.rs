use crate::engine::types::mesh::{Mesh, Triangle};

pub fn get_triangles_as_str(triangles: &Vec<Triangle>) -> String {
    let mut data = String::new();

    for t in triangles {
        let p1 = format!("{{{},{},{}}}\n", t.p1.x, t.p1.y, t.p1.z);
        data.push_str(&p1);
        let p2 = format!("{{{},{},{}}}\n", t.p2.x, t.p2.y, t.p2.z);
        data.push_str(&p2);
        let p3 = format!("{{{},{},{}}}\n", t.p3.x, t.p3.y, t.p3.z);
        data.push_str(&p3);
    }

    data
}

pub fn mesh_to_obj(mesh: &Mesh) -> String {
    let mut data = String::new();

    for &v in mesh.get_vertices(){
        let v_string = format!("v {} {} {}\n", v.x, v.y, v.z);
        data.push_str(&v_string);
    }

    for &f in mesh.get_faces(){
        let f_string = format!("f {} {} {}\n", f[0]+1, f[1]+1, f[2]+1);
        data.push_str(&f_string);
    }

    data
}
