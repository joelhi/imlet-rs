use std::{fs, path::Path};

pub enum Material{
    Normal,
    Arctic,
    InsideOutside,
}

impl Material {
    pub fn path(&self) -> &'static str {
        match self {
            Material::Normal => "normal.wgsl",
            Material::Arctic => "arctic.wgsl",
            Material::InsideOutside => "inside_outside.wgsl"
        }
    }
    
    pub fn load_shader_source(&self) -> String {
        let path = Path::new(file!()).parent().unwrap().join("shaders").join(self.path());
        println!("{}", path.to_str().unwrap());
        fs::read_to_string(path)
            .expect("Failed to read shader source file")
    }
}