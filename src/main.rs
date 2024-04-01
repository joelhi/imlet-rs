use std::time::Instant;

use implicit::engine::types::*;
use implicit::engine::utils::implicit_functions::{DistanceFunction, GyroidFunction};

use implicit::engine::algorithms::marching_cubes::generate_iso_surface;
use implicit::engine::utils::io::write_as_obj;

use implicit::viewer::window::run;

fn main() {
    let size = 50.0;
    let num_pts = 25;

    let mut grid = DenseGridF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    let _distance_func = DistanceFunction {
        source: XYZ::origin(),
    };
    let gyroid = GyroidFunction {
        length_x: 10.0,
        length_y: 10.0,
        length_z: 10.0,
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

    write_as_obj(&mesh, "output");

    println!("Running viewer...");
    pollster::block_on(run(&mesh));

}
