use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{functions::*, DenseFieldF32, Mesh, XYZ},
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 10;
    let size = 10.0;

    // Function
    let sphere_function = Sphere {
        source: XYZ::new(size / 2.0, size / 2.0, size / 2.0),
        radius: size * 0.45,
    };

    // Design space
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / ((num_pts - 1) as f32),
        num_pts,
        num_pts,
        num_pts,
    );
    grid.evaluate(&sphere_function, true);
    // Generate mesh
    let triangles = generate_iso_surface(&grid, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    // Run viewer
    pollster::block_on(run(&mesh, Material::Normal));
}
