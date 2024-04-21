

use crate::{engine::{algorithms::marching_cubes::generate_iso_surface, types::{functions::Sphere, DenseFieldF32, Mesh, XYZ}, utils}, viewer::{material::Material, window::run}};

pub fn run_sphere(num_pts: usize, size: f32){
    utils::logging::init();
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );


    let sphere_function = Sphere {
        source: XYZ::origin(),
        radius: size * 0.9,
    };

    grid.evaluate(&sphere_function, true);

    let triangles = generate_iso_surface(&grid, 0.0);

    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Normal));
}