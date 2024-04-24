use std::time::Instant;

use itertools::iproduct;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use super::XYZ;
use crate::engine::types::functions::ImplicitFunction;

#[derive(Debug, Clone)]
pub struct DenseFieldF32 {
    origin: XYZ,
    cell_size: f32,
    num_x: usize,
    num_y: usize,
    num_z: usize,

    buffer: Vec<f32>,
}

impl DenseFieldF32 {
    pub fn new(
        origin: XYZ,
        cell_size: f32,
        size_x: usize,
        sixe_y: usize,
        size_z: usize,
    ) -> DenseFieldF32 {
        DenseFieldF32 {
            origin: origin,
            cell_size: cell_size,
            num_x: size_x,
            num_y: sixe_y,
            num_z: size_z,
            buffer: Vec::with_capacity(size_x * sixe_y * size_z),
        }
    }

    pub fn evaluate<T: ImplicitFunction + Sync>(&mut self, function: &T, parallel: bool) {
        if parallel {
            self.evaluate_parallel(function)
        } else {
            self.evaluate_single(function)
        }
    }

    pub fn evaluate_single<T: ImplicitFunction>(&mut self, function: &T) {
        let before = Instant::now();
        self.buffer.clear();
        for k in 0..self.num_z {
            for j in 0..self.num_y {
                for i in 0..self.num_x {
                    let value = function.eval(
                        self.origin.x + self.cell_size * i as f32,
                        self.origin.y + self.cell_size * j as f32,
                        self.origin.z + self.cell_size * k as f32,
                    );
                    self.buffer.push(value);
                }
            }
        }

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            self.get_num_points(),
            before.elapsed()
        );
    }

    pub fn evaluate_parallel<T: ImplicitFunction + Sync>(&mut self, function: &T) {
        let before = Instant::now();
        let coordinates: Vec<(usize, usize, usize)> =
            iproduct!(0..self.num_x, 0..self.num_y, 0..self.num_z).collect();

        let local_buffer: Vec<(usize, f32)> = coordinates
            .into_par_iter()
            .map(|(i, j, k)| {
                let x = self.origin.x + self.cell_size * i as f32;
                let y = self.origin.y + self.cell_size * j as f32;
                let z = self.origin.z + self.cell_size * k as f32;
                let value = function.eval(x, y, z);

                (self.get_point_index(i, j, k), value)
            })
            .collect();

        self.buffer.clear();
        self.buffer.resize(self.get_num_points(), 0.0);
        for (index, value) in local_buffer {
            self.buffer[index] = value;
        }

        log::info!(
            "Dense value buffer for {} points generated in {:.2?}",
            self.get_num_points(),
            before.elapsed()
        );
    }

    pub fn smooth(&mut self, factor: f32, iterations: u32) {
        let before = Instant::now();
        for _ in 0..iterations {
            let smoothed_values: Vec<_> = self.buffer.clone()
                .into_par_iter()
                .enumerate()
                .map(|(index, val)| {
                    let smoothed_val = if let Some(sum) = self.get_neighbours_sum(index) {
                        let laplacian = sum / 6.0;
                        (1.0 - factor) * val + factor * laplacian
                    } else {
                        val
                    };
                    (index, smoothed_val)
                })
                .collect();
    
            for (index, smoothed_val) in smoothed_values {
                self.buffer[index] = smoothed_val;
            }
        }
        
        log::info!(
            "Dense value buffer for {} points smoothed in {:.2?} for {} iterations",
            self.get_num_points(),
            before.elapsed(),
            iterations
        );
    }

    pub fn threshold(&mut self, limit: f32){
        for value in self.buffer.iter_mut() {
            if *value < limit {
                *value = 0.0;
            }
        }
    }

    fn get_neighbours_sum(&self, index: usize) -> Option<f32> {
        let (i, j, k) = self.get_point_coord(index);

        if i < 1 || j < 1 || k < 1 || i == self.num_x -1 || j == self.num_y -1 || k == self.num_z -1 {
            return None;
        }
        Some(
            self.buffer[self.get_point_index(i + 1, j, k)] +
            self.buffer[self.get_point_index(i - 1, j, k)] +
            self.buffer[self.get_point_index(i, j + 1, k)] +
            self.buffer[self.get_point_index(i, j - 1, k)] +
            self.buffer[self.get_point_index(i, j, k + 1)] +
            self.buffer[self.get_point_index(i, j, k - 1)]
        )
    }

    pub fn get_cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if !i < self.num_x - 1 || !j < self.num_y - 1 || !k < self.num_z - 1 {
            panic!("Index out of bounds");
        }
        [
            self.get_point_index(i, j, k),
            self.get_point_index(i + 1, j, k),
            self.get_point_index(i + 1, j + 1, k),
            self.get_point_index(i, j + 1, k),
            self.get_point_index(i, j, k + 1),
            self.get_point_index(i + 1, j, k + 1),
            self.get_point_index(i + 1, j + 1, k + 1),
            self.get_point_index(i, j + 1, k + 1),
        ]
    }

    pub fn get_cell_data(&self, i: usize, j: usize, k: usize) -> ([XYZ; 8], [f32; 8]) {
        (self.get_cell_xyz(i, j, k), self.get_cell_values(i, j, k))
    }

    pub fn get_cell_xyz(&self, i: usize, j: usize, k: usize) -> [XYZ; 8] {
        let size = self.cell_size;
        let i_val = i as f32;
        let j_val = j as f32;
        let k_val = k as f32;
        [
            XYZ {
                x: i_val * size,
                y: j_val * size,
                z: k_val * size,
            },
            XYZ {
                x: (i_val + 1.0) * size,
                y: j_val * size,
                z: k_val * size,
            },
            XYZ {
                x: (i_val + 1.0) * size,
                y: (j_val + 1.0) * size,
                z: k_val * size,
            },
            XYZ {
                x: i_val * size,
                y: (j_val + 1.0) * size,
                z: k_val * size,
            },
            XYZ {
                x: i_val * size,
                y: j_val * size,
                z: (k_val + 1.0) * size,
            },
            XYZ {
                x: (i_val + 1.0) * size,
                y: j_val * size,
                z: (k_val + 1.0) * size,
            },
            XYZ {
                x: (i_val + 1.0) * size,
                y: (j_val + 1.0) * size,
                z: (k_val + 1.0) * size,
            },
            XYZ {
                x: i_val * size,
                y: (j_val + 1.0) * size,
                z: (k_val + 1.0) * size,
            },
        ]
    }

    pub fn get_cell_values(&self, i: usize, j: usize, k: usize) -> [f32; 8] {
        let cell_ids = self.get_cell_ids(i, j, k);
        [
            self.buffer[cell_ids[0]],
            self.buffer[cell_ids[1]],
            self.buffer[cell_ids[2]],
            self.buffer[cell_ids[3]],
            self.buffer[cell_ids[4]],
            self.buffer[cell_ids[5]],
            self.buffer[cell_ids[6]],
            self.buffer[cell_ids[7]],
        ]
    }

    pub fn get_point_index(&self, i: usize, j: usize, k: usize) -> usize {
        assert!(
            i < self.num_x && j < self.num_y && k < self.num_z,
            "Coordinates out of bounds"
        );
        (k * self.num_x * self.num_y) + (j * self.num_x) + i
    }

    pub fn get_point_coord(&self, index: usize) -> (usize, usize, usize) {
        assert!(index < self.get_num_points(), "Index out of bounds");
        let k = index / (self.num_x * self.num_y);
        let temp = index - (k * self.num_x * self.num_y);
        let j = temp / self.num_x;
        let i = temp % (self.num_x );

        (i, j, k)
    }

    pub fn get_cell_index(&self, i: usize, j: usize, k: usize) -> usize {
        assert!(
            i < self.num_x && j < self.num_y && k < self.num_z,
            "Coordinates out of bounds"
        );
        (k * (self.num_x - 1) * (self.num_y - 1)) + (j * (self.num_x - 1)) + i
    }

    pub fn get_cell_coord(&self, index: usize) -> (usize, usize, usize) {
        assert!(index < self.get_num_points(), "Index out of bounds");
        let k = index / ((self.num_x - 1) * (self.num_y - 1));
        let temp = index - (k * (self.num_x - 1) * (self.num_y - 1));
        let j = temp / (self.num_x - 1);
        let i = temp % (self.num_x - 1);

        (i, j, k)
    }

    pub fn get_num_points(&self) -> usize {
        self.num_x * self.num_y * self.num_z
    }

    pub fn get_num_cells(&self) -> usize {
        (self.num_x - 1) * (self.num_y - 1) * (self.num_z - 1)
    }
}
