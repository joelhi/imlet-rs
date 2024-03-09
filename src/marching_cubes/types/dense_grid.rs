use crate::XYZ;

use super::implicit_functions::base::ImplicitFunction;

#[derive(Debug, Clone, Copy)]
pub struct DenseGrid3f {
    pub origin: XYZ,
    pub cell_size: f32,
    pub num_x: usize,
    pub num_y: usize,
    pub num_z: usize,
}

impl DenseGrid3f {
    pub fn evaluate<T: ImplicitFunction>(self, function: &T) -> (Vec<XYZ>, Vec<f32>) {
        // Evaluate the function at all positions
        let mut coord: Vec<XYZ> = Vec::new();
        let mut values: Vec<f32> = Vec::new();
        for k in 0..self.num_x {
            for j in 0..self.num_y {
                for i in 0..self.num_z {
                    let temp_coord = XYZ {
                        x: self.cell_size * i as f32,
                        y: self.cell_size * j as f32,
                        z: self.cell_size * k as f32,
                    };
                    coord.push(temp_coord);
                    values.push(function.eval(temp_coord));
                }
            }
        }

        (coord, values)
    }

    pub fn get_cell_ids(self, i: usize, j: usize, k: usize) -> [usize; 8] {
        // Get the ids of the vertices at a certain cell
        if (!i < self.num_x - 1 || !j < self.num_y - 1 || !k < self.num_z - 1) {
            panic!("Index out of bounds");
        }
        [
            self.get_index(i, j, k),
            self.get_index(i + 1, j, k),
            self.get_index(i + 1, j + 1, k),
            self.get_index(i, j + 1, k),
            self.get_index(i, j, k + 1),
            self.get_index(i + 1, j, k + 1),
            self.get_index(i + 1, j + 1, k + 1),
            self.get_index(i, j + 1, k + 1),
        ]
    }

    pub fn get_index(self, i: usize, j: usize, k: usize) -> usize {
        i + self.num_x * (j + self.num_y * k)
    }

    pub fn get_size(self) -> usize {
        self.num_x * self.num_y * self.num_z
    }
}
