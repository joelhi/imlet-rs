use std::fs;
use std::time::Instant;

use implicit::types::grid::DenseGrid3f;
use implicit::types::core::*;

use implicit::utils::implicit_functions::{DistanceFunction, GyroidFunction};

use implicit::algorithms::marching_cubes::generate_iso_surface;

fn main() {
    let size = 10.0;
    let num_pts = 500;

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
        length_x: 2.5,
        length_y: 2.5,
        length_z: 2.5,
    };

    let before = Instant::now();
    grid.evaluate(&gyroid);

    println!(
        "Dense value buffer for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    let triangles = generate_iso_surface(&grid, 2.5);

    println!(
        "Full isosurface for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    fs::write("data.txt", get_triangle_as_str(&triangles)).expect("Unable to write file");
}

fn get_triangle_as_str(triangles: &Vec<Triangle>) -> String {
    let mut data = String::new();

    for t in triangles {
        let p1 = format!("{{{},{},{}}}\n", t.p1.x, t.p1.y, t.p1.z);
        data.push_str(&p1);
        let p2 = format!("{{{},{},{}}}\n", t.p2.x, t.p2.y, t.p2.z);
        data.push_str(&p2);
        let p3 = format!("{{{},{},{}}}\n", t.p3.x, t.p3.y, t.p3.z);
        data.push_str(&p3);
    }

    data
}
