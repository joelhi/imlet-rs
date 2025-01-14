use imlet::{
    types::geometry::Mesh,
    utils::{self, io::parse_obj_file},
    viewer,
};

pub fn main() {
    utils::logging::init_info();

    #[cfg(feature = "viewer")]
    {
        let mesh: Mesh<f32> = parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();
        viewer::show_mesh(&mesh, Some(mesh.bounds()));
    }
}
