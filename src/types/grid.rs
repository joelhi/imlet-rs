use super::core::XYZ;
use super::core::ImplicitFunction;

#[derive(Debug, Clone)]
pub struct DenseGrid3f {
    origin: XYZ,
    cell_size: f32,
    num_x: usize,
    num_y: usize,
    num_z: usize,

    buffer: Vec<f32>,
}

impl DenseGrid3f {
    pub fn new(
        origin: XYZ,
        cell_size: f32,
        size_x: usize,
        sixe_y: usize,
        size_z: usize,
    ) -> DenseGrid3f {
        DenseGrid3f {
            origin: origin,
            cell_size: cell_size,
            num_x: size_x,
            num_y: sixe_y,
            num_z: size_z,
            buffer: Vec::with_capacity(size_x * sixe_y * size_z),
        }
    }

    pub fn evaluate<T: ImplicitFunction>(&mut self, function: &T) {
        self.buffer.clear();
        // Evaluate the function at all positions
        for k in 0..self.num_x {
            for j in 0..self.num_y {
                for i in 0..self.num_z {
                    self.buffer.push(function.eval(
                        self.origin.x + self.cell_size * i as f32,
                        self.origin.y + self.cell_size * j as f32,
                        self.origin.z + self.cell_size * k as f32,
                    ));
                }
            }
        }
    }

    pub fn get_cell_ids(&self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if !i < self.num_x - 1 || !j < self.num_y - 1 || !k < self.num_z - 1 {
            panic!("Index out of bounds");
        }
        [
            self.get_cell_index(i, j, k),
            self.get_cell_index(i + 1, j, k),
            self.get_cell_index(i + 1, j + 1, k),
            self.get_cell_index(i, j + 1, k),
            self.get_cell_index(i, j, k + 1),
            self.get_cell_index(i + 1, j, k + 1),
            self.get_cell_index(i + 1, j + 1, k + 1),
            self.get_cell_index(i, j + 1, k + 1),
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
                x: i_val + 1.0 * size,
                y: j_val * size,
                z: k_val * size,
            },
            XYZ {
                x: i_val + 1.0 * size,
                y: j_val + 1.0 * size,
                z: k_val * size,
            },
            XYZ {
                x: i_val * size,
                y: j_val + 1.0 * size,
                z: k_val * size,
            },
            XYZ {
                x: i_val * size,
                y: j_val * size,
                z: k_val + 1.0 * size,
            },
            XYZ {
                x: i_val + 1.0 * size,
                y: j_val * size,
                z: k_val + 1.0 * size,
            },
            XYZ {
                x: i_val + 1.0 * size,
                y: j_val + 1.0 * size,
                z: k_val + 1.0 * size,
            },
            XYZ {
                x: i_val * size,
                y: j_val + 1.0 * size,
                z: k_val + 1.0 * size,
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

    pub fn get_point_index3(&self, index: usize) -> (usize, usize, usize) {
        assert!(index < self.get_num_points(), "Index out of bounds");
        let k = index / (self.num_x * self.num_y);
        let temp = index - (k * self.num_x * self.num_y);
        let j = temp / self.num_x;
        let i = temp % (self.num_x-1);

        (i, j, k)
    }

    pub fn get_cell_index(&self, i: usize, j: usize, k: usize) -> usize {
        assert!(
            i < self.num_x && j < self.num_y && k < self.num_z,
            "Coordinates out of bounds"
        );
        (k * (self.num_x-1) * (self.num_y-1)) + (j * (self.num_x-1)) + i
    }

    pub fn get_cell_coord(&self, index: usize) -> (usize, usize, usize) {
        assert!(index < self.get_num_points(), "Index out of bounds");
        let k = index / ((self.num_x-1) * (self.num_y-1));
        let temp = index - (k * (self.num_x-1) * (self.num_y-1));
        let j = temp / (self.num_x-1);
        let i = temp % (self.num_x-1);

        (i, j, k)
    }

    pub fn get_num_points(&self) -> usize {
        self.num_x * self.num_y * self.num_z
    }

    pub fn get_num_cells(&self) -> usize {
       (self.num_x - 1) * (self.num_y - 1) * (self.num_z - 1)
    }
}
