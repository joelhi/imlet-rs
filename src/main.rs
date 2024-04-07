use std::time::Instant;

use implicit::engine::types::*;
use implicit::engine::utils::implicit_functions::{DistanceFunction, GyroidFunction};

use implicit::engine::algorithms::marching_cubes::generate_iso_surface;
use implicit::engine::utils::io::write_as_obj;

use implicit::viewer::material::Material;
use implicit::viewer::window::run;

fn main() {

    let size = 50.0;
    let num_pts = 100;

    let mut grid = DenseGridF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    let _distance_func = DistanceFunction {
        source: XYZ {
            x: size / 2.0,
            y: size / 2.0,
            z: size / 2.0,
        },
    };
    let _gyroid = GyroidFunction {
        length_x: 30.0,
        length_y: 7.5,
        length_z: 15.0,
    };

    let before = Instant::now();
    grid.evaluate(&_gyroid);

    println!(
        "Dense value buffer for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    let triangles = generate_iso_surface(&grid, 0.0);

    let mesh = Mesh::from_triangles(&triangles);

    println!(
        "Full isosurface for {} points generated in {:.2?} with {} vertices and {} faces",
        grid.get_num_points(),
        before.elapsed(),
        mesh.num_vertices(),
        mesh.num_faces()
    );

    write_as_obj(&mesh, "output");

    println!("Running viewer...");
    pollster::block_on(run(&mesh, Material::InsideOutside));
}