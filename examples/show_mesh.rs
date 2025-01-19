pub fn main() {
    env_logger::init();
    #[cfg(feature = "viewer")]
    {
        let mesh: imlet::types::geometry::Mesh<f32> =
            imlet::utils::io::parse_obj_file("assets/geometry/bunny.obj", false, false).unwrap();
        let settings = imlet::viewer::DisplaySettings {
            show_bounds: true,
            show_mesh_edges: true,
            mesh_material: imlet::viewer::Material::Normal,
        };
        imlet::viewer::show_mesh_with_settings(&mesh, Some(mesh.bounds()), &settings);
    }
    #[cfg(not(feature = "viewer"))]
    {
        log::error!("Enable the viewer feature to see the model.");
    }
}
