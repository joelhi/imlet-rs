use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            functions::{Difference, Gyroid, Intersection, Offset, Sphere},
            DenseFieldF32, Mesh, XYZ,
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let num_pts = 200;
    let size = 10.0;
    let length = 2.5;

    // Design space
    let mut grid = DenseFieldF32::new(
        XYZ::origin(),
        size / ((num_pts - 1) as f32),
        num_pts,
        num_pts,
        num_pts,
    );

    // Function
    let gyroid = Gyroid::with_equal_spacing(length);
    let offset = Offset::new(gyroid, -0.80);
    let diff = Difference::new(offset, gyroid);
    let sphere = Sphere::new(XYZ::new(size/2.0,size/2.0,size/2.0), 0.45 * size);
    let union = Intersection::new(sphere, diff);

    grid.evaluate(&union, true);
    grid.threshold(0.1);
    grid.smooth(0.75, 10);

    // Generate output
    let triangles = generate_iso_surface(&grid, 0.15);
    let mesh = Mesh::from_triangles(&triangles);

    // Run viewer
    pollster::block_on(run(&mesh, Material::Normal));
}
