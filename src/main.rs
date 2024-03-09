mod marching_cubes;

use marching_cubes::types::{core::*, dense_grid::DenseGrid3f};

use crate::marching_cubes::types::implicit_functions::examples::DistanceFunction;

fn main() {
    let grid = DenseGrid3f {
        origin: XYZ::get_origin(),
        cell_size: 1.0,
        num_x: 5,
        num_y: 5,
        num_z: 5,
    };

    let cell_ids = grid.get_cell_ids(0, 0, 0);

    let distance_func = DistanceFunction{source: XYZ::get_origin()};

    let (coord, values) = grid.evaluate(&distance_func);

    println!("{:#?}",coord.len());
    println!("{:#?}",values.len());
}
