use super::material::Material;

pub struct DisplaySettings {
    pub show_bounds: bool,
    pub show_mesh_edges: bool,
    pub mesh_material: Material,
}

impl DisplaySettings {
    pub fn new() -> Self {
        Self {
            show_bounds: true,
            show_mesh_edges: true,
            mesh_material: Material::Normal,
        }
    }
}
