use imlet::utils;

pub fn main() {
    utils::logging::init_info();
    #[cfg(feature = "viewer")]
    {
        let mesh: imlet::types::geometry::Mesh<f32> =
            utils::io::parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();
        imlet::viewer::show_mesh(&mesh, Some(mesh.bounds()));
    }
}
