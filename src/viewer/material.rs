use std::{fs, path::Path};

pub enum Material {
    Normal,
    Arctic,
    InsideOutside,
    Line,
}

impl Material {
    pub fn path(&self) -> &'static str {
        match self {
            Material::Normal => "assets/shaders/normal.wgsl",
            Material::Arctic => "assets/shaders/arctic.wgsl",
            Material::InsideOutside => "assets/shaders/inside_outside.wgsl",
            Material::Line => "assets/shaders/line.wgsl",
        }
    }

    pub fn load_shader_source(&self) -> String {
        fs::read_to_string(path).expect("Failed to read shader source file")
    }
}