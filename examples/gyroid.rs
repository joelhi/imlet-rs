use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            functions::{Difference, Gyroid, Offset},
            DenseFieldF32, Mesh, XYZ,
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 150;
    let size = 10.0;
    let length = 2.5;

    // Design space
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / (num_pts as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    // Function
    let gyroid = Gyroid::with_equal_spacing(length);
    let offset = Offset::new(gyroid, -0.1);
    let diff = Difference::new(offset, gyroid);

    grid.evaluate(&diff, true);

    // Generate output
    let triangles = generate_iso_surface(&grid, 0.15);
    let mesh = Mesh::from_triangles(&triangles);

    // Run viewer
    pollster::block_on(run(&mesh, Material::Normal));
}
