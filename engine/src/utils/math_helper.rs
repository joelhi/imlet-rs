use num_traits::Float;
pub trait Pi {
    fn pi() -> Self;
}

impl Pi for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }
}

impl Pi for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }
}

pub fn normalize<T: Float>(value: T, min: T, max: T) -> T {
    (value - min) / (max - min)
}

pub fn index1d_from_index3d(
    i: usize,
    j: usize,
    k: usize,
    num_x: usize,
    num_y: usize,
    num_z: usize,
) -> usize {
    assert!(
        i < num_x && j < num_y && k < num_z,
        "Coordinates out of bounds"
    );
    (k * num_x * num_y) + (j * num_x) + i
}

pub fn index3d_from_index1d(
    index: usize,
    num_x: usize,
    num_y: usize,
    num_z: usize,
) -> (usize, usize, usize) {
    assert!(index < num_x * num_y * num_z, "Index out of bounds");
    let k = index / (num_x * num_y);
    let temp = index - (k * num_x * num_y);
    let j = temp / num_x;
    let i = temp % num_x;

    (i, j, k)
}