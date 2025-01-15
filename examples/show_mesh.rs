use imlet::utils;

pub fn main() {
    env_logger::init();
    #[cfg(feature = "viewer")]
    {
        let mesh: imlet::types::geometry::Mesh<f32> =
            utils::io::parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();
        imlet::viewer::show_mesh(&mesh, Some(mesh.bounds()));
    }
    #[cfg(not(feature = "viewer"))]
    {
        log::error!("Enable the viewer feature to see the model.");
    }
}
