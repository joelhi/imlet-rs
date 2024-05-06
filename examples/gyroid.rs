

use implicit::{
    engine::{
        algorithms::marching_cubes::generate_iso_surface,
        types::{
            computation::{
                functions::{Gyroid, Sphere},
                operations::{
                    shape::Offset,
                    boolean::{Difference, Intersection},
                },
                Model,
            },
            geometry::{BoundingBox, Mesh, Vec3f},
        },
        utils,
    },
    viewer::{material::Material, window::run},
};

pub fn main() {
    utils::logging::init();

    // Inputs
    let size = 10.0;
    let cell_size = 0.05;
    let bounds = BoundingBox::new(Vec3f::origin(), Vec3f::new(size, size, size));

    // Build model
    let mut model = Model::new();

    let sphere = model.add_function(Sphere::new(
        Vec3f::new(size / 2.0, size / 2.0, size / 2.0),
        size * 0.45,
    ));
    let gyroid = model.add_function(Gyroid::with_equal_spacing(2.5));
    let offset_gyroid = model.add_operation(Offset::new(gyroid, -0.8));
    let subtracted_gyroid = model.add_operation(Difference::new(gyroid, offset_gyroid));
    let union = model.add_operation(Intersection::new(sphere, subtracted_gyroid));

    // Discretize
    let mut field = model.evaluate(bounds, cell_size, union);

    field.smooth(0.75, 10);

    // Generate mesh
    let triangles = generate_iso_surface(&field, 0.0);
    let mesh = Mesh::from_triangles(&triangles);

    pollster::block_on(run(&mesh, Material::Arctic));
}
