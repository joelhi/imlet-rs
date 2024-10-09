use bevy::app::App;

use super::plugins::MeshViewerPlugin;

/// Open an interactive window which renders a mesh object with orbit controls.
/// 
/// # Arguments
///
/// * `mesh` - The mesh to render.
///
pub fn show_mesh(mesh: &crate::types::geometry::Mesh<f32>) {
    App::new()
        .add_plugins(MeshViewerPlugin::new(mesh))
        .run();
}