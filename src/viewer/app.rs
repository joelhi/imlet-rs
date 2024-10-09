use bevy::app::App;
use num_traits::Float;

use super::plugins::MeshViewerPlugin;

/// Open an interactive window which renders a mesh object with orbit controls.
///
/// # Arguments
///
/// * `mesh` - The mesh to render.
///
pub fn show_mesh<T: Float>(mesh: &crate::types::geometry::Mesh<T>) {
    App::new().add_plugins(MeshViewerPlugin::new(&mesh.convert::<f32>())).run();
}
