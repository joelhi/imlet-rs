#[derive(Debug, Clone, Copy)]
pub struct Vec3i {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3i {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Vec3i { x: x, y: y, z: z }
    }

    pub fn product(&self) -> usize {
        self.x * self.y * self.z
    }
}

impl Into<(usize, usize, usize)> for Vec3i {
    fn into(self) -> (usize, usize, usize) {
        (self.x, self.y, self.z)
    }
}

impl From<(usize, usize, usize)> for Vec3i {
    fn from(tuple: (usize, usize, usize)) -> Self {
        let (x, y, z) = tuple;
        Vec3i { x, y, z }
    }
}
