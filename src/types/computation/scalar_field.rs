use std::fmt::Debug;
use std::time::Instant;

use num_traits::Float;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;

use crate::types::geometry::Vec3;
use crate::types::geometry::Vec3i;
use crate::utils::math_helper::index1d_from_index3d;
use crate::utils::math_helper::index3d_from_index1d;

/// 3-dimensional field for scalar values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalarField<T> {
    origin: Vec3<T>,
    cell_size: T,
    n: Vec3i,
    data: Vec<T>,
}

impl<T> ScalarField<T> {
    /// Create a new field and populate it with data.
    ///
    /// The size of the data buffer must match the point count.
    ///
    /// # Arguments
    ///
    /// * `origin` - The base of the field, and the first value location in space.
    /// * `cell_size` - The size of each cell in the field.
    /// * `num_pts` - Number of points in each direction.
    /// * `data` - The data buffer.
    pub fn with_data(origin: Vec3<T>, cell_size: T, num_pts: Vec3i, data: Vec<T>) -> Self {
        if num_pts.product() != data.len() {
            panic!("Incorrect size of data buffer");
        }
        Self {
            origin,
            cell_size,
            n: num_pts,
            data,
        }
    }

    /// Returns the origin of the field.
    pub fn origin(&self) -> &Vec3<T> {
        &self.origin
    }

    /// Returns the totla number of points in the field.
    pub fn num_points(&self) -> usize {
        self.n.product()
    }

    /// Returns the total number of cells in the field.
    pub fn num_cells(&self) -> usize {
        (self.n.i - 1) * (self.n.j - 1) * (self.n.k - 1)
    }

    /// Returns a slice of the data buffer in the field.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    fn cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if !i < self.n.i - 1 || !j < self.n.j - 1 || !k < self.n.k - 1 {
            panic!("Index out of bounds");
        }
        [
            self.point_index1d(i, j, k),
            self.point_index1d(i + 1, j, k),
            self.point_index1d(i + 1, j + 1, k),
            self.point_index1d(i, j + 1, k),
            self.point_index1d(i, j, k + 1),
            self.point_index1d(i + 1, j, k + 1),
            self.point_index1d(i + 1, j + 1, k + 1),
            self.point_index1d(i, j + 1, k + 1),
        ]
    }

    pub(crate) fn point_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        index1d_from_index3d(i, j, k, self.n.i, self.n.j, self.n.k)
    }

    pub(crate) fn point_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.i, self.n.j, self.n.k)
    }

    pub(crate) fn cell_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.i - 1, self.n.j - 1, self.n.k - 1)
    }
}

impl<T: Float> ScalarField<T> {
    /// Create a new empty field.
    /// # Arguments
    ///
    /// * `origin` - The base of the field, and the first data location.
    /// * `cell_size` - The size of each cell in the field.
    /// * `num_pts` - Number of points in each direction.
    pub fn new(origin: Vec3<T>, cell_size: T, num_pts: Vec3i) -> Self {
        Self {
            origin,
            cell_size,
            n: num_pts,
            data: vec![T::zero(); num_pts.product()],
        }
    }

    /// Returns the cell size of the field.
    pub fn cell_size(&self) -> T {
        self.cell_size
    }

    /// Returns a copy of the data buffer in the field.
    pub fn copy_data(&self) -> Vec<T> {
        self.data.clone()
    }

    /// Returns the vertex locations at the corners of the specified cell.
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    pub fn cell_corners(&self, i: usize, j: usize, k: usize) -> [Vec3<T>; 8] {
        let size = self.cell_size;
        let i_val = T::from(i).expect("Failed to convert number to T");
        let j_val = T::from(j).expect("Failed to convert number to T");
        let k_val = T::from(k).expect("Failed to convert number to T");
        let one = T::one();
        [
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: (j_val + one) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: (j_val + one) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: j_val * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: j_val * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: (i_val + one) * size,
                    y: (j_val + one) * size,
                    z: (k_val + one) * size,
                },
            self.origin
                + Vec3 {
                    x: i_val * size,
                    y: (j_val + one) * size,
                    z: (k_val + one) * size,
                },
        ]
    }

    /// Returns the values at the corners of the specified cell.
    /// # Arguments
    ///
    /// * `i` - Index in first direction.
    /// * `j` - Index in second direction.
    /// * `k` - Index in third direction.
    pub fn cell_values(&self, i: usize, j: usize, k: usize) -> [T; 8] {
        let cell_ids = self.cell_ids(i, j, k);
        [
            self.data[cell_ids[0]],
            self.data[cell_ids[1]],
            self.data[cell_ids[2]],
            self.data[cell_ids[3]],
            self.data[cell_ids[4]],
            self.data[cell_ids[5]],
            self.data[cell_ids[6]],
            self.data[cell_ids[7]],
        ]
    }

    /// Assigns 0 to any point with an absolute value below the limit.
    /// # Arguments
    ///
    /// * `limit` - The limit threshold for non-zero values.
    pub fn threshold(&mut self, limit: T) {
        self.data.iter_mut().for_each(|value| {
            if (*value).abs() < limit {
                *value = T::zero();
            }
        });
    }

    fn neighbours_sum(&self, index: usize) -> Option<T> {
        let (i, j, k) = self.point_index3d(index);

        if i < 1 || j < 1 || k < 1 || i == self.n.i - 1 || j == self.n.j - 1 || k == self.n.k - 1 {
            return None;
        }
        Some(
            self.data[self.point_index1d(i + 1, j, k)]
                + self.data[self.point_index1d(i - 1, j, k)]
                + self.data[self.point_index1d(i, j + 1, k)]
                + self.data[self.point_index1d(i, j - 1, k)]
                + self.data[self.point_index1d(i, j, k + 1)]
                + self.data[self.point_index1d(i, j, k - 1)],
        )
    }

    /// Performs a laplacian smoothing operation on the field data.
    ///
    /// The value of each point will be updated based on the average of the adjacent points.
    /// # Arguments
    ///
    /// * `factor` - Interpolation value between the average of the adjacent points and the current value.
    /// * `iterations` - Number of successive smoothing iterations.
    pub fn smooth(&mut self, factor: T, iterations: u32) {
        let before = Instant::now();
        let mut smoothed = vec![T::zero(); self.num_points()];
        for _ in 0..iterations {
            smoothed.iter_mut().enumerate().for_each(|(index, val)| {
                if let Some(sum) = self.neighbours_sum(index) {
                    let laplacian = sum / T::from(6.0).expect("Failed to convert number to T");
                    *val = (T::one() - factor) * self.data[index] + factor * laplacian;
                } else {
                    *val = self.data[index];
                };
            });
            std::mem::swap(&mut self.data, &mut smoothed);
        }

        log::info!(
            "Dense value data for {} points smoothed in {:.2?} for {} iterations",
            self.num_points(),
            before.elapsed(),
            iterations
        );
    }
}

impl<T: Float + Send + Sync> ScalarField<T> {
    /// Performs a laplacian smoothing operation on the field data using parallel iteration.
    ///
    /// The value of each point will be updated based on the average of the adjacent points.
    /// # Arguments
    ///
    /// * `factor` - Interpolation value between the average of the adjacent points and the current value.
    /// * `iterations` - Number of successive smoothing iterations.
    pub fn smooth_par(&mut self, factor: T, iterations: u32) {
        let before = Instant::now();
        let mut smoothed = vec![T::zero(); self.num_points()];
        for _ in 0..iterations {
            smoothed
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, val)| {
                    if let Some(sum) = self.neighbours_sum(index) {
                        let laplacian = sum / T::from(6.0).expect("Failed to convert number to T");
                        *val = (T::one() - factor) * self.data[index] + factor * laplacian;
                    } else {
                        *val = self.data[index];
                    };
                });
            std::mem::swap(&mut self.data, &mut smoothed);
        }

        log::info!(
            "Dense value data for {} points smoothed in {:.2?} for {} iterations",
            self.num_points(),
            before.elapsed(),
            iterations
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_field_half() {
        let mut data = vec![1.0; 27];
        data[13] = 2.0;
        let mut field = ScalarField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.smooth(0.5, 1);

        let field_data = field.copy_data();

        for (idx, val) in field_data.iter().enumerate() {
            if idx == 13 {
                assert!((1.5 - val).abs() < 0.001);
            } else {
                assert!((1.0 - val).abs() < 0.001);
            }
        }
    }

    #[test]
    fn test_smooth_field_full() {
        let mut data = vec![1.0; 27];
        data[13] = 2.0;
        let mut field = ScalarField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.smooth(1.0, 1);

        let field_data = field.copy_data();

        for val in field_data {
            assert!((1.0 - val).abs() < 0.001);
        }
    }

    #[test]
    fn test_smooth_field_full_varied() {
        let mut data = vec![0.0; 27];
        data[4] = 10.0;
        data[10] = 20.0;
        data[12] = 5.0;
        data[14] = 20.0;
        data[16] = 15.0;
        data[22] = 20.0;
        let mut field = ScalarField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.smooth(1.0, 1);

        let field_data = field.copy_data();
        assert!((15.0 - field_data[13]).abs() < 0.001);
    }

    #[test]
    fn test_smooth_field_half_varied() {
        let mut data = vec![0.0; 27];
        data[4] = 10.0;
        data[10] = 20.0;
        data[12] = 5.0;
        data[14] = 20.0;
        data[16] = 15.0;
        data[22] = 20.0;
        let mut field = ScalarField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.smooth(0.5, 1);

        let field_data = field.copy_data();
        assert!((7.5 - field_data[13]).abs() < 0.001);
    }

    #[test]
    fn test_threshold_field() {
        let mut data = vec![0.01; 27];
        data[13] = 2.0;
        data[20] = 1.0;
        data[21] = 1.5;

        let mut field = ScalarField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.threshold(0.1);

        let field_data = field.copy_data();

        assert_eq!(24, field_data.iter().filter(|&val| *val == 0.0).count());
        assert!((2.0 - field_data[13]).abs() < 0.001);
        assert!((1.0 - field_data[20]).abs() < 0.001);
        assert!((1.5 - field_data[21]).abs() < 0.001);
    }

    #[test]
    fn test_map_point_index() {
        let field = ScalarField::new(Vec3::origin(), 1.0, (10, 10, 10).into());

        assert_eq!(1, field.point_index1d(1, 0, 0));
        assert_eq!(10, field.point_index1d(0, 1, 0));
        assert_eq!(11, field.point_index1d(1, 1, 0));
        assert_eq!(100, field.point_index1d(0, 0, 1));
        assert_eq!(110, field.point_index1d(0, 1, 1));
        assert_eq!(111, field.point_index1d(1, 1, 1));
    }
}
