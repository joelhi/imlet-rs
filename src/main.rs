mod marching_cubes;

use std::time::Instant;

use marching_cubes::{algorithm::generate_iso_surface, types::{core::*, dense_grid::DenseGrid3f}};

use crate::marching_cubes::types::implicit_functions::examples::DistanceFunction;

fn main() {
    let grid = DenseGrid3f {
        origin: XYZ::get_origin(),
        cell_size: 1.0,
        num_x: 10,
        num_y: 10,
        num_z: 10,
    };

    let distance_func = DistanceFunction{source: XYZ::get_origin()};

    let (coord, values) = grid.evaluate(&distance_func);

    let triangles = generate_iso_surface(grid, 5.0, &coord, &values);

    for t in triangles{
        println!("{{{},{},{}}}",t.p1.x,t.p1.y,t.p1.z);
        println!("{{{},{},{}}}",t.p2.x,t.p2.y,t.p2.z);
        println!("{{{},{},{}}}",t.p3.x,t.p3.y,t.p3.z);
    }
}
