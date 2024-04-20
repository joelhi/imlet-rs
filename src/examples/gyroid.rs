use std::time::Instant;

use crate::{engine::{algorithms::marching_cubes::generate_iso_surface, types::{DenseFieldF32, Mesh, XYZ}, utils::implicit_functions::{GyroidFunction, ImplicitDifference, ImplicitOffset}}, viewer::{material::Material, window::run}};

pub fn run_gyroid(num_pts: usize, size: f32, length: f32){
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    let _gyroid = GyroidFunction {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let _gyroid2 = GyroidFunction {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let offset = ImplicitOffset{
        f: _gyroid2,
        distance: -0.1
    };

    let diff = ImplicitDifference{
        f: offset,
        g: _gyroid
    };

    let before = Instant::now();
    grid.evaluate(&diff, true);

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