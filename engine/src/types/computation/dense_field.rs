use std::fmt::Debug;
use std::time::Instant;

use num_traits::Float;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::types::geometry::Vec3;
use crate::types::geometry::Vec3i;
use crate::utils::math_helper::index1d_from_index3d;
use crate::utils::math_helper::index3d_from_index1d;

#[derive(Debug, Clone)]
pub struct DenseField<T: Float + Debug> {
    origin: Vec3<T>,
    cell_size: T,
    n: Vec3i,
    data: Vec<T>,
}

impl<T: Float + Debug + Send + Sync> DenseField<T> {
    pub fn with_data(origin: Vec3<T>, cell_size: T, num_pts: Vec3i, data: Vec<T>) -> Self {
        if num_pts.product() != data.len() {
            panic!("Incorrect size of data buffer");
        }
        Self {
            origin: origin,
            cell_size: cell_size,
            n: num_pts,
            data: data,
        }
    }

    pub fn new(origin: Vec3<T>, cell_size: T, num_pts: Vec3i) -> Self {
        Self {
            origin: origin,
            cell_size: cell_size,
            n: num_pts,
            data: vec![T::zero(); num_pts.product()],
        }
    }

    pub fn smooth(&mut self, factor: T, iterations: u32) {
        let before = Instant::now();
        let mut smoothed =
            vec![T::zero(); self.get_num_points()];
        for _ in 0..iterations {
            smoothed
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, val)| {
                    if let Some(sum) = self.get_neighbours_sum(index) {
                        let laplacian = sum / T::from(6.0).expect("Failed to convert number to T");
                        *val = (T::one() - factor)
                            * self.data[index]
                            + factor * laplacian;
                    } else {
                        *val = self.data[index];
                    };
                });
            std::mem::swap(&mut self.data, &mut smoothed);
        }

        log::info!(
            "Dense value data for {} points smoothed in {:.2?} for {} iterations",
            self.get_num_points(),
            before.elapsed(),
            iterations
        );
    }

    pub fn threshold(&mut self, limit: T) {
        self.data.iter_mut().for_each(|value| {
            if *value < limit {
                *value = T::zero();
            }
        });
    }

    fn get_neighbours_sum(&self, index: usize) -> Option<T> {
        let (i, j, k) = self.get_point_index3d(index);

        if i < 1 || j < 1 || k < 1 || i == self.n.x - 1 || j == self.n.y - 1 || k == self.n.z - 1 {
            return None;
        }
        Some(
            self.data[self.get_point_index1d(i + 1, j, k)]
                + self.data[self.get_point_index1d(i - 1, j, k)]
                + self.data[self.get_point_index1d(i, j + 1, k)]
                + self.data[self.get_point_index1d(i, j - 1, k)]
                + self.data[self.get_point_index1d(i, j, k + 1)]
                + self.data[self.get_point_index1d(i, j, k - 1)],
        )
    }

    fn get_cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if !i < self.n.x - 1 || !j < self.n.y - 1 || !k < self.n.z - 1 {
            panic!("Index out of bounds");
        }
        [
            self.get_point_index1d(i, j, k),
            self.get_point_index1d(i + 1, j, k),
            self.get_point_index1d(i + 1, j + 1, k),
            self.get_point_index1d(i, j + 1, k),
            self.get_point_index1d(i, j, k + 1),
            self.get_point_index1d(i + 1, j, k + 1),
            self.get_point_index1d(i + 1, j + 1, k + 1),
            self.get_point_index1d(i, j + 1, k + 1),
        ]
    }

    pub fn get_cell_corners(&self, i: usize, j: usize, k: usize) -> [Vec3<T>; 8] {
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

    pub fn get_cell_values(&self, i: usize, j: usize, k: usize) -> [T; 8] {
        let cell_ids = self.get_cell_ids(i, j, k);
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

    pub fn get_point_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        index1d_from_index3d(i, j, k, self.n.x, self.n.y, self.n.z)
    }

    pub fn get_point_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.x, self.n.y, self.n.z)
    }

    pub fn get_cell_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        index1d_from_index3d(i, j, k, self.n.x - 1, self.n.y - 1, self.n.z - 1)
    }

    pub fn get_cell_index3d(&self, index: usize) -> (usize, usize, usize) {
        index3d_from_index1d(index, self.n.x - 1, self.n.y - 1, self.n.z - 1)
    }

    pub fn get_num_points(&self) -> usize {
        self.n.x * self.n.y * self.n.z
    }

    pub fn get_num_cells(&self) -> usize {
        (self.n.x - 1) * (self.n.y - 1) * (self.n.z - 1)
    }

    pub fn copy_data(&self) -> Vec<T> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_field_half() {
        let mut data = vec![1.0; 27];
        data[13] = 2.0;
        let mut field = DenseField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
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
        let mut field = DenseField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
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
        let mut field = DenseField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
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
        let mut field = DenseField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
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

        let mut field = DenseField::with_data(Vec3::origin(), 1.0, (3, 3, 3).into(), data);
        field.threshold(0.1);

        let field_data = field.copy_data();

        assert_eq!(24, field_data.iter().filter(|&val| *val == 0.0).count());
        assert!((2.0 - field_data[13]).abs() < 0.001);
        assert!((1.0 - field_data[20]).abs() < 0.001);
        assert!((1.5 - field_data[21]).abs() < 0.001);
    }

    #[test]
    fn test_map_cell_index_cube() {
        let field = DenseField::new(Vec3::origin(), 1.0, (10, 10, 10).into());

        assert_eq!(1, field.get_cell_index1d(1, 0, 0));
        assert_eq!(9, field.get_cell_index1d(0, 1, 0));
        assert_eq!(10, field.get_cell_index1d(1, 1, 0));
        assert_eq!(81, field.get_cell_index1d(0, 0, 1));
        assert_eq!(90, field.get_cell_index1d(0, 1, 1));
        assert_eq!(91, field.get_cell_index1d(1, 1, 1));
    }

    #[test]
    fn test_map_point_index() {
        let field = DenseField::new(Vec3::origin(), 1.0, (10, 10, 10).into());

        assert_eq!(1, field.get_point_index1d(1, 0, 0));
        assert_eq!(10, field.get_point_index1d(0, 1, 0));
        assert_eq!(11, field.get_point_index1d(1, 1, 0));
        assert_eq!(100, field.get_point_index1d(0, 0, 1));
        assert_eq!(110, field.get_point_index1d(0, 1, 1));
        assert_eq!(111, field.get_point_index1d(1, 1, 1));
    }
}
