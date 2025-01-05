use num_traits::Float;

/// Additional trait for generic computation using PI. Used for periodic surfaces such as Gyroids.
pub trait Pi {
    /// Returns the value of Pi.
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

#[inline]
pub(crate) fn normalize<T: Float>(value: T, min: T, max: T) -> T {
    (value - min) / (max - min)
}

#[inline]
pub(crate) fn index1d_from_index3d(
    i: usize,
    j: usize,
    k: usize,
    num_x: usize,
    num_y: usize,
    num_z: usize,
) -> usize {
    debug_assert!(
        i < num_x && j < num_y && k < num_z,
        "Coordinates out of bounds"
    );
    (k * num_x * num_y) + (j * num_x) + i
}

#[inline]
pub(crate) fn index3d_from_index1d(
    index: usize,
    num_x: usize,
    num_y: usize,
    num_z: usize,
) -> (usize, usize, usize) {
    debug_assert!(index < num_x * num_y * num_z, "Index out of bounds");
    let k = index / (num_x * num_y);
    let temp = index - (k * num_x * num_y);
    let j = temp / num_x;
    let i = temp % num_x;

    (i, j, k)
}

#[inline]
pub(crate) fn format_integer(n: usize) -> String {
    let mut s = n.to_string();
    let len = s.len();
    for i in (1..len).rev() {
        if (len - i) % 3 == 0 {
            s.insert(i, ',');
        }
    }
    s
}
