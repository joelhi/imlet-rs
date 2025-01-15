use std::fs;

/// Materials that can be used for rendering meshes.
pub enum Material {
    /// Shading based on mesh normals.
    Normal,
    /// Simple smooth shading based on view projection.
    Arctic,
    /// Toon shader with different colour on inside and outside of geometry.
    InsideOutside,
    /// Constant colour, used for lines.
    Line,
}

impl Material {
    pub(crate) fn path(&self) -> &'static str {
        match self {
            Material::Normal => "assets/shaders/normal.wgsl",
            Material::Arctic => "assets/shaders/arctic.wgsl",
            Material::InsideOutside => "assets/shaders/inside_outside.wgsl",
            Material::Line => "assets/shaders/line.wgsl",
        }
    }

    pub(crate) fn load_shader_source(&self) -> String {
        fs::read_to_string(self.path()).expect("Failed to read shader source file")
    }
}
