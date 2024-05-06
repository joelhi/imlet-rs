#[derive(Debug, Clone, Copy)]
pub struct Vec3i {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3i {
    pub fn new(x: usize, y: usize, z: usize)->Self{
        Vec3i {
            x: x,
            y: y,
            z: z,
        }
    }
}