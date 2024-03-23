use std::fs;
use std::time::Instant;

use implicit::types::grid::DenseGrid3f;
use implicit::types::core::*;

use implicit::types::mesh::Mesh;
use implicit::utils::implicit_functions::{DistanceFunction, GyroidFunction};

use implicit::algorithms::marching_cubes::generate_iso_surface;
use implicit::utils::io::mesh_to_obj;

fn main() {
    let size = 10.0;
    let num_pts = 25;

    let mut grid = DenseGrid3f::new(
        XYZ::get_origin(),
        size / num_pts as f32,
        num_pts,
        num_pts,
        num_pts,
    );

    let _distance_func = DistanceFunction {
        source: XYZ::get_origin(),
    };
    let gyroid = GyroidFunction {
        length_x: 3.0,
        length_y: 3.0,
        length_z: 3.0,
    };

    let before = Instant::now();
    grid.evaluate(&gyroid);

    println!(
        "Dense value buffer for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    let triangles = generate_iso_surface(&grid, 0.0);

    let mesh = Mesh::from_triangles(&triangles);

    println!(
        "Full isosurface for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    fs::write("data.obj", mesh_to_obj(&mesh)).expect("Unable to write file");
    //fs::write("data.txt", get_triangles_as_str(&triangles)).expect("Unable to write file");
}
