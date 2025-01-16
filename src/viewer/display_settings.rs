use super::material::Material;

/// Struct to store some settings for the viewer.
pub struct DisplaySettings {
    /// Set to true to show the bounding box
    pub show_bounds: bool,
    /// Set to true to render the mesh edges.
    pub show_mesh_edges: bool,
    /// Material used to render the mesh.
    pub mesh_material: Material,
}

impl DisplaySettings {
    /// Create new settings with default properties.
    pub fn new() -> Self {
        Self {
            show_bounds: true,
            show_mesh_edges: false,
            mesh_material: Material::Normal,
        }
    }

    /// Create new settings with a specific material.
    pub fn with_material(material: Material) -> Self {
        Self {
            show_bounds: true,
            show_mesh_edges: true,
            mesh_material: material,
        }
    }
}
