use std::time::Instant;

use crate::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{DenseFieldF32, Mesh, XYZ},
        utils::implicit_functions::{
            GyroidFunction, ImplicitDifference, ImplicitIntersection, ImplicitOffset, ImplicitSmoothUnion, Sphere
        },
    }, viewer::{material::Material, window::run}
};

pub fn run_bitmask_gyroid(num_pts: usize, size: f32, length: f32, parallel: bool) {
    let sphere1 = Sphere {
        source: XYZ {
            x: size / 2.0,
            y: size / 4.0,
            z: size / 2.0,
        },
        radius: 0.25 * size,
    };

    let sphere2 = Sphere {
        source: XYZ {
            x: size / 2.0,
            y: size / 2.0,
            z: size / 3.0,
        },
        radius: 0.25 * size,
    };

    let gyroid = GyroidFunction {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let gyroid2 = GyroidFunction {
        length_x: length,
        length_y: length,
        length_z: length,
    };

    let offset = ImplicitOffset{
        f: gyroid2,
        distance: -0.5
    };

    let thick_gyroid = ImplicitDifference{
        f: offset,
        g: gyroid
    };

    let spheres = ImplicitSmoothUnion{
        f: sphere1,
        g: sphere2
    };

    let final_func = ImplicitIntersection{
        f: thick_gyroid,
        g: spheres
    };

    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    let before = Instant::now();
    grid.evaluate(&final_func, parallel);

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

    println!("Running viewer...");
    pollster::block_on(run(&mesh, Material::Normal));
}
