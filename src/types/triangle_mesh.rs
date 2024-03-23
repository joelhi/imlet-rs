use super::core::XYZ;

pub struct TriangleMesh{
    pub triangles:Vec<Triangle>
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p1: XYZ,
    pub p2: XYZ,
    pub p3: XYZ,
}