use std::fs;

pub enum Material{
    Normal,
    Arctic,
    InsideOutside,
}

impl Material {
    pub fn path(&self)->&'static str{
        match self {
            Material::Normal =>"src/viewer/shaders/normal.wgsl",
            Material::Arctic =>"src/viewer/shaders/arctic.wgsl",
            Material::InsideOutside =>"src/viewer/shaders/inside_outside.wgsl"
        }
    }
    pub fn load_shader_source(&self) -> String {
        fs::read_to_string(self.path())
            .expect("Failed to read shader source file")
    }
}