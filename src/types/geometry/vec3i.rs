#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Vector with three integer coordinates. Mainly used for 3d indexing.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub struct Vec3i {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}

impl Vec3i {
    /// Create a new Vec3i from three indexes.
    /// * `i` - First index.
    /// * `j` - Second index.
    /// * `k` - Third index.
    pub fn new(i: usize, j: usize, k: usize) -> Self {
        Vec3i { i, j, k }
    }

    /// Compute the product of the indexes. (i * j * k)
    pub fn product(&self) -> usize {
        self.i * self.j * self.k
    }
}

impl From<Vec3i> for (usize, usize, usize) {
    fn from(val: Vec3i) -> Self {
        (val.i, val.j, val.k)
    }
}

impl From<(usize, usize, usize)> for Vec3i {
    fn from(tuple: (usize, usize, usize)) -> Self {
        let (i, j, k) = tuple;
        Vec3i { i, j, k }
    }
}
