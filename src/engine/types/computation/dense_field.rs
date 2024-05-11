use std::time::Instant;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::engine::types::geometry::Vec3f;
use crate::engine::types::geometry::Vec3i;

#[derive(Debug, Clone)]
pub struct DenseFieldF32 {
    origin: Vec3f,
    cell_size: f32,
    cell_n: Vec3i,
    data: Vec<f32>,
}

impl DenseFieldF32 {
    pub fn new(origin: Vec3f, cell_size: f32, cell_n: Vec3i, data: Vec<f32>) -> DenseFieldF32 {
        if cell_n.product() != data.len() {
            panic!("Incorrect size of data buffer");
        }
        DenseFieldF32 {
            origin: origin,
            cell_size: cell_size,
            cell_n: cell_n,
            data: data,
        }
    }

    pub fn empty(origin: Vec3f, cell_size: f32, cell_n: Vec3i) -> DenseFieldF32 {
        DenseFieldF32 {
            origin: origin,
            cell_size: cell_size,
            cell_n: cell_n,
            data: vec![0.0; cell_n.product()],
        }
    }

    pub fn smooth(&mut self, factor: f32, iterations: u32) {
        let before = Instant::now();
        let mut smoothed = vec![0.0; self.get_num_points()];
        for _ in 0..iterations {
            smoothed
                .par_iter_mut()
                .enumerate()
                .for_each(|(index, val)| {
                    if let Some(sum) = self.get_neighbours_sum(index) {
                        let laplacian = sum / 6.0;
                        *val = (1.0 - factor) * self.data[index] + factor * laplacian;
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

    pub fn threshold(&mut self, limit: f32) {
        self.data.iter_mut().for_each(|value| {
            if *value < limit {
                *value = 0.0;
            }
        });
    }

    fn get_neighbours_sum(&self, index: usize) -> Option<f32> {
        let (i, j, k) = self.get_point_index3d(index);

        if i < 1
            || j < 1
            || k < 1
            || i == self.cell_n.x - 1
            || j == self.cell_n.y - 1
            || k == self.cell_n.z - 1
        {
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

    pub fn get_cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if !i < self.cell_n.x - 1 || !j < self.cell_n.y - 1 || !k < self.cell_n.z - 1 {
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

    pub fn get_cell_data(&self, i: usize, j: usize, k: usize) -> ([Vec3f; 8], [f32; 8]) {
        (self.get_cell_vec3f(i, j, k), self.get_cell_values(i, j, k))
    }

    pub fn get_cell_vec3f(&self, i: usize, j: usize, k: usize) -> [Vec3f; 8] {
        let size = self.cell_size;
        let i_val = i as f32;
        let j_val = j as f32;
        let k_val = k as f32;
        [
            self.origin
                + Vec3f {
                    x: i_val * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3f {
                    x: (i_val + 1.0) * size,
                    y: j_val * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3f {
                    x: (i_val + 1.0) * size,
                    y: (j_val + 1.0) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3f {
                    x: i_val * size,
                    y: (j_val + 1.0) * size,
                    z: k_val * size,
                },
            self.origin
                + Vec3f {
                    x: i_val * size,
                    y: j_val * size,
                    z: (k_val + 1.0) * size,
                },
            self.origin
                + Vec3f {
                    x: (i_val + 1.0) * size,
                    y: j_val * size,
                    z: (k_val + 1.0) * size,
                },
            self.origin
                + Vec3f {
                    x: (i_val + 1.0) * size,
                    y: (j_val + 1.0) * size,
                    z: (k_val + 1.0) * size,
                },
            self.origin
                + Vec3f {
                    x: i_val * size,
                    y: (j_val + 1.0) * size,
                    z: (k_val + 1.0) * size,
                },
        ]
    }

    pub fn get_cell_values(&self, i: usize, j: usize, k: usize) -> [f32; 8] {
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
        DenseFieldF32::index1d_from_index3d(i, j, k, self.cell_n.x, self.cell_n.y, self.cell_n.z)
    }

    pub fn get_point_index3d(&self, index: usize) -> (usize, usize, usize) {
        DenseFieldF32::index3d_from_index1d(index, self.cell_n.x, self.cell_n.y, self.cell_n.z)
    }

    pub fn get_cell_index1d(&self, i: usize, j: usize, k: usize) -> usize {
        DenseFieldF32::index1d_from_index3d(
            i,
            j,
            k,
            self.cell_n.x - 1,
            self.cell_n.y - 1,
            self.cell_n.z - 1,
        )
    }

    pub fn get_cell_index3d(&self, index: usize) -> (usize, usize, usize) {
        DenseFieldF32::index3d_from_index1d(
            index,
            self.cell_n.x - 1,
            self.cell_n.y - 1,
            self.cell_n.z - 1,
        )
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

    pub fn get_num_points(&self) -> usize {
        self.cell_n.x * self.cell_n.y * self.cell_n.z
    }

    pub fn get_num_cells(&self) -> usize {
        (self.cell_n.x - 1) * (self.cell_n.y - 1) * (self.cell_n.z - 1)
    }
}
