mod marching_cubes;

use std::{fs, time::Instant};

use marching_cubes::{algorithm::generate_iso_surface, types::{core::*, dense_grid::DenseGrid3f}};

use crate::marching_cubes::types::implicit_functions::examples::{DistanceFunction, GyroidFunction};

fn main() {
    let grid = DenseGrid3f {
        origin: XYZ::get_origin(),
        cell_size: 1.0,
        num_x: 10,
        num_y: 10,
        num_z: 10,
    };

    let distance_func = DistanceFunction{source: XYZ::get_origin()};
    let gyroid = GyroidFunction{
        length_x: 5.0,
        length_y: 5.0,
        length_z: 5.0
    };

    let before = Instant::now();
    let (coord, values) = grid.evaluate(&gyroid);

    let triangles = generate_iso_surface(grid, 0.0, &coord, &values);

    println!("Isosurface for {} points generated in {:.2?}", grid.get_size(), before.elapsed());

    fs::write("data.txt", get_triangle_as_str(&triangles)).expect("Unable to write file");

}

fn get_triangle_as_str(triangles: &Vec<Triangle>)->String{
    let mut data = String::new();

        for t in triangles{
            let p1 = format!("{{{},{},{}}}\n",t.p1.x,t.p1.y,t.p1.z);
            data.push_str(&p1);
            let p2 = format!("{{{},{},{}}}\n",t.p2.x,t.p2.y,t.p2.z);
            data.push_str(&p2);
            let p3 = format!("{{{},{},{}}}\n",t.p3.x,t.p3.y,t.p3.z);
            data.push_str(&p3);
        }

        data
}
