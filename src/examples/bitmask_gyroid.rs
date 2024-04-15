use std::time::Instant;

use crate::{engine::{algorithms::marching_cubes::generate_iso_surface, types::{DenseGridF32, Mesh, XYZ}, utils::implicit_functions::{BitMask, GyroidFunction, ImplicitProduct, Sphere}}, viewer::{material::Material, window::run}};

pub fn run_bitmask_gyroid(num_pts: usize, size: f32, length: f32) {
    let _bounds = Sphere {
        source: XYZ{x: size/2.0, y: size/2.0, z: size/2.0},
        radius: 0.4*size,
    };

    let bounds = GyroidFunction {
        length_x: length * 5.0,
        length_y: length * 5.0,
        length_z: length * 5.0,
    };

    let gyroid = GyroidFunction {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let bit_mask = BitMask {
        function: bounds,
        cut_off: 0.0, 
    };

    let bit_mask2 = BitMask {
        function: _bounds,
        cut_off: 0.0, 
    };

    let product = ImplicitProduct{
        f: gyroid,
        g: bit_mask
    };

    let product2 = ImplicitProduct{
        f: product,
        g: bit_mask2
    };

    let mut grid = DenseGridF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    let before = Instant::now();
    grid.evaluate(&product2);

    println!(
        "Dense value buffer for {} points generated in {:.2?}",
        grid.get_num_points(),
        before.elapsed()
    );

    let triangles = generate_iso_surface(&grid, 0.15);

    let mesh = Mesh::from_triangles(&triangles);

    println!(
        "Full isosurface for {} points generated in {:.2?} with {} vertices and {} faces",
        grid.get_num_points(),
        before.elapsed(),
        mesh.num_vertices(),
        mesh.num_faces()
    );

    println!("Running viewer...");
    pollster::block_on(run(&mesh, Material::Normal));

}
